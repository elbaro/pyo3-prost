import timeit

import rupy_proto
from tweet_pb2 import Tweet as PythonTweet
from tweet_pb2 import User as PythonUser

RustTweet = rupy_proto.Tweet

tweet = PythonTweet()
tweet.text = 'Hi this is a text'
tweet.created_timestamp = 1232123
tweet.author.name = 'Who'
tweet.author.profile_url = 'https://example.com/'
tweet.mentions.append('@trump')
tweet.mentions.append('@obama')
bench_data = tweet.SerializeToString()

p = PythonTweet()
p.ParseFromString(bench_data)
print(
    'Python: ',
    timeit.timeit(
        lambda: p.SerializeToString(),
        number=int(1e6),
    ),
)

r = RustTweet.decode(bench_data)
print(
    'Rust  : ',
    timeit.timeit(lambda: r.encode(), number=int(1e6)),
)


print('===========')
print('PythonTweet')
print('===========')
print(p.SerializeToString())
print('===========')
print('  RustTweet')
print('===========')
print(r.encode())
