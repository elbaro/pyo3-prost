use prost_types::field::Cardinality;

pub enum FieldType {
    Unknown,
    String,
    I64,
    Message,
}

pub struct Field {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub cardinality: Cardinality,
    pub type_kind: FieldType,
    pub message_ident: Option<syn::Ident>,
}

fn extract_ident_from_ty(ty: &syn::Type) -> Option<syn::Ident> {
    let syn::Type::Path(ref typepath) = ty else { return None; };
    if typepath.qself.is_some() {
        return None;
    }
    // segments: core, option, Option<..>
    let last_seg = typepath.path.segments.last()?;
    Some(last_seg.ident.clone())
}

// a::b::c<d::e::f> -> d::e::f
fn extract_inner_type(ty: &syn::Type) -> Option<syn::Type> {
    let syn::Type::Path(ref typepath) = ty else { return None; };
    if typepath.qself.is_some() {
        return None;
    }
    // segments: core, option, Option<..>
    let last_seg = typepath.path.segments.last()?;
    let type_params = &last_seg.arguments;
    // It should have only on angle-bracketed param ("<String>"):
    match type_params {
        syn::PathArguments::AngleBracketed(params) => params.args.first(),
        _ => None,
    }
    .and_then(|generic_arg| match generic_arg {
        syn::GenericArgument::Type(ty) => Some(ty.clone()),
        _ => None,
    })
}

pub fn parse_fields(fields_: &syn::Fields) -> Vec<Field> {
    let mut fields = vec![];

    if let syn::Fields::Named(fields_named) = fields_ {
        for field in &fields_named.named {
            // attr = #[prost(string, repeated, tag = "5")]
            for attr in &field.attrs {
                if attr.path().is_ident("prost") {
                    // message_ident =
                    //     Message -> Message
                    //     Option<Message> -> Message
                    //     Vec<Message> -> Message
                    let mut field = Field {
                        ident: field.ident.clone().unwrap(),
                        ty: field.ty.clone(),
                        cardinality: Cardinality::Unknown,
                        type_kind: FieldType::Unknown,
                        message_ident: None,
                    };
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("repeated") {
                            field.cardinality = Cardinality::Repeated;
                        } else if meta.path.is_ident("optional") {
                            field.cardinality = Cardinality::Optional;
                        } else if meta.path.is_ident("required") {
                            field.cardinality = Cardinality::Required;
                        } else if meta.path.is_ident("string") {
                            field.type_kind = FieldType::String;
                        } else if meta.path.is_ident("int64") {
                            field.type_kind = FieldType::I64;
                        } else if meta.path.is_ident("message") {
                            field.type_kind = FieldType::Message;
                        } else if meta.path.is_ident("tag") {
                        } else {
                            eprintln!("unrecognized type: {:?}", meta.path);
                            return Err(meta.error("unrecognized type"));
                        }
                        Ok(())
                    });

                    if let FieldType::Message = field.type_kind {
                        let msg_ty = match field.cardinality {
                            Cardinality::Unknown | Cardinality::Optional => {
                                extract_inner_type(&field.ty).unwrap()
                            }
                            Cardinality::Required => field.ty.clone(),
                            Cardinality::Repeated => extract_inner_type(&field.ty).unwrap(),
                        };
                        field.message_ident = extract_ident_from_ty(&msg_ty);
                    }
                    fields.push(field);

                    break;
                }
            }
        }
    }

    fields
}
