print("Calculator in Python!")
a = int(input("Enter first number: "))
b = int(input("Enter second number: "))
def add():
    print(a + b)

def subtract():
    print(a - b)

def multiply():
    print(a * b)

def divide():
    print(a / b)

menu_dict = {
    "add": add,
    "subtract": subtract,
    "multiply": multiply,
    "divide": divide
}

print("1. add\n2. subtract\n3. multiply\n4. divide\n5. exit")
option = input("Enter option: ")
menu_dict[option]()