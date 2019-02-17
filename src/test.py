import rust_from_py
print(rust_from_py.sum_as_string(4,5))
print(rust_from_py.test_array([1,2,3,4]))
print(rust_from_py.test_return_array([1,2,3,4]))
print("Working Lambda", rust_from_py.test_lambda([1,2,3,4], lambda x:x*2))
print("Failing Lambda", rust_from_py.test_lambda([1,2,3,4], lambda x,y:x*2))