from rust_bindings import just_return, square_even_numbers_in_list, sum_as_string

if __name__ == "__main__":
    # -- sum_as_string
    a, b = 1, 2
    s = sum_as_string(a, b)
    print(f"Sum of {a} and {b} is: {s}")

    # -- square_even_numbers_in_list
    lst = [1, 2, 3, 4, 5, 6]
    lst_even_squared = square_even_numbers_in_list(lst)
    print(f"Only even numbers squared: {lst_even_squared}")

    # -- just_return
    value = 42
    result = just_return(value)
    print(f"Just return {value}: {result}")
