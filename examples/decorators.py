def add_drive_method(func):
    def drive(self):
        print(f"{self.model} поехала!")
        return func(self)
    return drive


class Car:
    def __init__(self, model, color):
        self.model = model
        self.color = color

    @add_drive_method
    def __str__(self):
        return f"Машина {self.model} цвета {self.color}"
    

if __name__ == "__main__":
    car = Car("BMW", "черный")
    print(car)