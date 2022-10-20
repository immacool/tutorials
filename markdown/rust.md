
# Изучение Rust

## Оглавление

1. [Переменные](#переменные)
    1. [Создание переменных](#создание-переменных)
    2. [Система типов](#система-типов)
        1. [Стандартные типы](#стандартные-типы-данных-в-rust)
        2. [Целые числа](#целые-числа)
        3. [Числа с плавающей точкой](#числа-с-плавающей-точкой)
        4. [Символы](#символы)
        5. [Булевы значения](#булевы-значения)
        6. [Строки](#строки)
        7. [Приведение типов](#приведение-типов)
    3. [Коллекции](#коллекции).
        1. [Вектор](#вектор)
        2. [Строки](#строка-как-колекция)
        3. [Хэш-таблица](#хэш-таблица)
        4. [Массив](#массив)
2. [Условные операторы](#условные-операторы)
    1. [if](#if)
    2. [if let](#if-let)
    3. [match](#match)
3. [Циклы](#циклы)
    1. [loop](#loop)
    2. [while](#while)
    3. [for](#for)
4. [Функции](#функции)
5. [Структура программы](#структура-программы)

* [Ссылки](#ссылки)
* [Сноски](#сноски)

## Переменные

### Создание переменных

Для создания переменных в Rust используется ключевое слово `let`:

```rust
let x = 5;
```

В языке Rust существует два типа переменных: изменяемые и неизменяемые. Переменные по умолчанию являются неизменяемыми, для того чтобы сделать переменную изменяемой, необходимо использовать ключевое слово `mut` перед её именем.

```rust
let x = 5; // неизменяемая переменная
let mut y = 10; // изменяемая переменная

x = 10; // ошибка компиляции
y = 15; // изменение значения переменной
```

#### Вывод в консоль

Для вывода в консоль используется функция `println!`:

```rust
println!("Hello, world!");
```

Вывод:

```text
Hello, world!
```

#### Форматирование вывода

Для форматирования вывода в `println!` используется макрос `format!`:

```rust
let x = 5;
let y = 10;

println!("x = {}, y = {}", x, y);
```

Вывод:

```text
x = 5, y = 10
```

Вы не видите `format!` в коде, потому что он вызывается внутри `println!`. В Rust макросы вызываются с помощью символа `!`.

Макрос - это функция, которая вызывается с восклицательным знаком в конце имени. Макросы позволяют создавать более сложные функции, чем обычные функции. Об этом подробнее можно прочитать в [документации](https://doc.rust-lang.org/book/ch19-06-macros.html) или в главе [Макросы](#макросы).

```rust
let x = 5;
let y = 10;

println!("x = {}, y = {}", x, y);
```

Так же, можно использовать сам `format!` для сохранения результата форматирования в переменную:

```rust
let s = format!("x = {}, y = {}", x, y);
```

И далее использовать эту переменную:

```rust
println!("{}", s);
```

### Система типов

В Rust существует система типов, которая позволяет компилятору проверять типы переменных во время компиляции. Это позволяет избежать ошибок во время выполнения программы.

```rust
let x = 5; // переменная типа i32
let y = 5.5; // переменная типа f64
let z = true; // переменная типа bool
```

Вам не обязательно указывать тип переменной, компилятор Rust сам определит тип переменной по её значению. Но можно указать тип переменной явно:

```rust
let x: i32 = 5;
let y: f64 = 5.5;
let z: bool = true;
```

#### Стандартные типы данных в Rust

* Целые числа: `1`, `2`, `3`, ...
  * `i8 - i128` - знаковые целые числа с 8 до 128 бит
  * `u8 - u128` - беззнаковые целые числа с 8 до 128 бит
* Числа с плавающей точкой: `1.0`, `2.0`, `3.0`, ...
  * `f32` - число с плавающей точкой 32 бита
  * `f64` - число с плавающей точкой 64 бита
* Символы: `'a'`, `'b'`, `'c'`, ...
  * `char` - символ в кодировке Unicode (4 байта)
* Булевы значения: `true`, `false`
  * `bool` - булево значение
* Строки: `"Hello, world!"`, `"Hello, Rust!"`, ...
  * `&str` - строка в кодировке UTF-8 (неизменяемая)
  * `String` - строка в кодировке UTF-8 (изменяемая)

Массивы, векторы, кортежи и другие типы данных будут рассмотрены в следующей главе [Коллекции](#коллекции).

#### Целые числа

Целые числа в Rust могут быть знаковыми или беззнаковыми. Знаковые числа могут быть отрицательными, а беззнаковые - только положительными.

```rust
let x = 5; // i32
let y = 5u8; // u8
let z = -5; // i32
```

> signed - знаковый тип, unsigned - беззнаковый тип

Для обозначения unsigned типа используется суффикс `u` и для обозначения signed типа используется суффикс `i`. Суффикс `u` может быть опущен, по умолчанию используется signed тип.

#### Числа с плавающей точкой

Числа с плавающей точкой в Rust могут быть `f32` или `f64`. По умолчанию используется `f64`.

```rust
let x = 5.5; // f64
let y = 5.5f32; // f32
```

Так же, как и с целыми числами, суффикс `f32` или `f64` может быть опущен, по умолчанию используется `f64`.

#### Символы

Символы в Rust обозначаются одинарными кавычками.

```rust
let x = 'x';
let two_hearts = '💕';
```

#### Булевы значения

Булевы значения в Rust могут быть только `true` или `false`.

```rust
let t = true;
let f: bool = false; // с явным указанием типа
```

#### Строки

Строки в Rust могут быть двух типов: `&str` и `String`.

* `&str` - это строка в кодировке UTF-8, которая не может быть изменена. Это неизменяемая строка.
* `String` - это строка в кодировке UTF-8, которая может быть изменена. Это изменяемая строка.

```rust
let x = "Hello, world!"; // &str
mut let y = String::from(x); // String
```

Строки в Rust являются неизменяемыми по умолчанию. Для создания изменяемой строки используется метод `String::from`. Метод `String::from` создает новую строку из строки в кодировке UTF-8. В примере выше создается изменяемая строка `y` из неизменяемой строки `x`. Подробнее о функциях будет сказано в главе [Функции](#функции).

#### Приведение типов

Приведение типов в Rust происходит с помощью функции `as`. Приведение типов может быть неявным или явным.

```rust
let x = 5; // i32
let y = 5.5; // f64

let a = x as f64;
let b = y as i32;
let c = x as u8;
```

Различают *неявное* и *явное* приведение типов. Неявное приведение типов происходит автоматически, когда компилятор может определить, что приведение типов безопасно. Явное приведение типов происходит с помощью функции `as`. В примере выше `y` приводится к типу `i32` неявно, так как это безопасно. `x` приводится к типу `u8` явно, так как это может привести к потере данных.

Потеря данных при переводе типа может произойти, если значение не может быть представлено в новом типе. Например, если мы попытаемся привести число `500` к типу `u8`, то потеряем данные, так как `u8` может хранить только значения от `0` до `255`.

```rust
let x = 500; // i32
let y = x as u8; // 244
```

При приведении типов в Rust не происходит автоматического приведения типов. Например, если мы попытаемся сложить число `5` и строку `"5"`, то получим ошибку компиляции.

```rust
let x = 5;
let y = "5";
let z = x + y; // error
```

Для того, чтобы сложить число и строку, необходимо привести число к строке. Для этого используется функция `to_string`.

```rust
let x = 5;
let y = "5";
let z = x.to_string() + y;
```

Вывод:

```text
55
```

И наоборот, для численного результата необходимо привести строку к числу. Для этого используется функция `parse`.

```rust
let x = 5;
let y = "5";
let z = x + y.parse::<i32>().unwrap();
```

Вывод:

```text
10
```

Разберем строку с преобразованием типов по частям.

* `y.parse::<i32>()` - преобразование строки `y` к типу `i32`. Возвращает `Result`, который содержит либо число, либо ошибку. Для того, чтобы получить число, необходимо вызвать метод `unwrap`. Если в результате преобразования произошла ошибка, то программа завершится с ошибкой.
* `.unwrap()` - вызов метода `unwrap` для `Result`. Если в результате преобразования произошла ошибка, то программа завершится с ошибкой.
* `x + y.parse::<i32>().unwrap()` - сложение числа `x` и числа, полученного из строки `y`.

`Result` - это тип, который используется для обработки ошибок. Рассмотрим его подробнее в главе [Обработка ошибок](#обработка-ошибок).

### Коллекции

Коллекции - это структуры данных, которые позволяют хранить несколько значений в одной переменной. В Rust есть три основных типа коллекций: вектор, строка и хэш-карта (словарь, хэш-таблица).

* Вектор - это изменяемая коллекция, которая хранит элементы одного типа. Вектор можно изменять, добавлять и удалять элементы. Вектор можно создать с помощью функции `vec!`.
* Строка - это изменяемая коллекция, которая хранит символы. Строка можно изменять, добавлять и удалять символы. Строка можно создать с помощью функции `String::new`.
* Хэш-карта - это изменяемая коллекция, которая хранит пары ключ-значение. В хэш-карте можно изменять, добавлять и удалять элементы. Хэш-карту можно создать с помощью функции `HashMap::new`.
* Массив - это неизменяемая коллекция, которая хранит элементы одного типа. Массив можно создать с помощью функции `array!`.

Разница между массивом и вектором в том, что массив имеет фиксированный размер, а вектор может изменяться. Векторы чаще всего используются, поскольку их удобнее использовать. Массивы используются в тех случаях, когда необходимо хранить элементы одного типа и заранее известно количество элементов. Так же массивы считаются быстрее, чем вектора.

#### Вектор

```rust
// Создание вектора
let v = vec![1, 2, 3];

// Добавление элемента в вектор
v.push(4);

// Удаление последнего элемента в векторе
v.pop();

// Удаление элемента из вектора по индексу
v.remove(0);

// Получение элемента из вектора
let x = v[0]; // 1

// Получение элемента из вектора с проверкой наличия элемента
let x = v.get(0); // Some(1)
let x = v.get(10); // None

// Изменение элемента в векторе
v[0] = 5;
```

Разберем пример по порядку:

* `let v = vec![1, 2, 3];` - создание вектора с тремя элементами. `vec!` - это макрос, который создает вектор.
* `v.push(4);` - добавление элемента `4` в вектор `v` при помощи метода `push`.
* `v.pop();` - удаление последнего элемента в векторе `v` при помощи метода `pop`.
* `v.remove(0);` - удаление элемента с индексом `0` в векторе `v` при помощи метода `remove`.
* `let x = v[0];` - получение первого элемента вектора `v` при помощи индекса `0`. Индексация в векторе начинается с нуля.
* `let x = v.get(0);` - получение первого элемента вектора `v` при помощи индекса `0` при помощи метода `get`. Метод `get` возвращает `Option`, поэтому возвращаемое значение - `Some(1)`. Если элемента с таким индексом нет, то возвращается `None`. Подробнее о `Option` можно почитать в [документации](https://doc.rust-lang.org/std/option/).
* `v[0] = 5;` - изменение первого элемента вектора `v` на `5`.

#### Строка как колекция

```rust
// Создание строки
let s = String::new();

// Добавление символа в строку
s.push('a');

// Добавление строки в строку
s.push_str("abc");

// Удаление последнего символа в строке
s.pop();

// Удаление конкретного символа в строке
s.remove(0);

// Получение символа из строки
let x = s[0]; // 'a'

// Получение символа из строки с проверкой наличия символа
let x = s.get(0); // Some('a')
let x = s.get(10); // None

// Изменение символа в строке
s[0] = 'b';
```

Разберем пример по порядку:

* `let s = String::new();` - создание пустой строки. `String::new` - это метод, который создает пустую строку.
* `s.push('a');` - добавление символа `a` в строку `s` при помощи метода `push`.
* `s.push_str("abc");` - добавление строки `"abc"` в строку `s` при помощи метода `push_str`.
* `s.pop();` - удаление последнего символа в строке `s` при помощи метода `pop`.
* `s.remove(0);` - удаление символа с индексом `0` в строке `s` при помощи метода `remove`.
* `let x = s[0];` - получение первого символа строки `s` при помощи индекса `0`. Индексация в строке начинается с нуля.
* `let x = s.get(0);` - получение первого символа строки `s` при помощи индекса `0` при помощи метода `get`.
* `s[0] = 'b';` - изменение первого символа строки `s` на `b`.

#### Хэш-таблица

```rust
// Создание хэш-таблицы
let mut h = HashMap::new();

// Добавление пары ключ-значение в хэш-таблицу
h.insert("a", 1);

// Удаление пары ключ-значение из хэш-таблицы
h.remove("a");

// Получение значения из хэш-таблицы
let x = h["a"]; // 1

// Получение значения из хэш-таблицы с проверкой наличия значения
let x = h.get("a"); // Some(1)
let x = h.get("b"); // None

// Изменение значения в хэш-таблице
h["a"] = 2;
```

Разберем пример по порядку:

* `let mut h = HashMap::new();` - создание пустой хэш-таблицы. `HashMap::new` - это метод, который создает пустую хэш-таблицу.
* `h.insert("a", 1);` - добавление пары ключ-значение `("a", 1)` в хэш-таблицу `h` при помощи метода `insert`. Ключом может быть любой тип, реализующий `Hash` и `Eq`, а значением - любой тип. Так же, это можно сделать при помощи оператора `=`, например `h["a"] = 1;`.
* `h.remove("a");` - удаление пары ключ-значение `("a", 1)` из хэш-таблицы `h` при помощи метода `remove`.
* `let x = h["a"];` - получение значения из хэш-таблицы `h` при помощи ключа `"a"`. Ключи в хэш-таблице могут быть только строками.
* `let x = h.get("a");` - получение значения из хэш-таблицы `h` при помощи ключа `"a"` при помощи метода `get`.
* `h["a"] = 2;` - изменение значения из хэш-таблицы `h` при помощи ключа `"a"` на `2`.

#### Массив

```rust
// Создание пустого массива
let mut a = array![];

// Создание массива с определенным размером
let mut a = array![0; 10];

// Создание массива
let a = [1, 2, 3];

// Получение значения из массива
let x = a[0]; // 1

// Изменение значения в массиве
a[0] = 2;
```

Разберем пример по порядку:

* `let mut a = array![];` - создание пустого массива. `array![]` - это макрос, который создает пустой массив.
* `let mut a = array![0; 10];` - создание массива с определенным размером. `array![0; 10]` - это макрос, который создает массив из 10 элементов со значением `0`.
* `let a = [1, 2, 3];` - создание массива.
* `let x = a[0];` - получение значения из массива `a` при помощи индекса `0`.
* `a[0] = 2;` - изменение значения из массива `a` при помощи индекса `0` на `2`.

## Условные операторы

В языке Rust есть три вида условных операторов: `if`, `if let` и `match`.

### if

Обычный `if` выглядит так:

```rust
if условие {
    // код
}
```

Так же, можно использовать `else`:

```rust
if условие {
    // код
} else {
    // код
}
```

Или `else if`:

```rust
if условие {
    // код
} else if условие {
    // код
} else {
    // код
}
```

Пример кода:

```rust
let x = 1;

if x > 0 {
    println!("x > 0");
} else if x < 0 {
    println!("x < 0");
} else {
    println!("x = 0");
}
```

### if let

`if let` позволяет проверить значение переменной и выполнить код, если оно соответствует определенному шаблону.

```rust
if let шаблон = значение {
    // код
}
```

Пример кода:

```rust
let x = Some(1);

if let Some(x) = x {
    println!("x = {}", x);
}
```

### match

`match` позволяет проверить значение переменной и выполнить код, если оно соответствует определенному шаблону.

```rust
match значение {
    шаблон => код,
    шаблон => код,
    ...
}
```

Пример кода:

```rust
let x = Some(1);

match x {
    Some(x) => println!("x = {}", x),
    None => println!("x = None"),
}
```

Так же в `match` можно использовать `_`:

```rust
let x = Some(1);

match x {
    Some(x) => println!("x = {}", x),
    _ => println!("x = None"),
}
```

`_` - это шаблон, который соответствует любому не предусмотренному шаблону.

## Циклы

В языке Rust есть три вида циклов: `loop`, `while` и `for`.

### loop

`loop` - это бесконечный цикл.

```rust
loop {
    // код
}
```

Пример кода:

```rust
loop {
    println!("Hello, World!");
}
```

### while

`while` - это цикл с условием, который выполняется, пока условие истинно.

```rust
while условие {
    // код
}
```

Пример кода:

```rust
let mut x = 0;

while x < 10 {
    println!("x = {}", x);
    x += 1;
}
```

### for

`for` - это цикл, который выполняется для каждого элемента в коллекции.

```rust
for элемент in коллекция {
    // код
}
```

Пример кода:

```rust
let a = [1, 2, 3];

for x in a {
    println!("x = {}", x);
}
```

Так же, можно использовать `range`[^1], чтобы получить последовательность чисел:

```rust
for x in 0..10 {
    println!("x = {}", x);
}
```

#### Range

`Range` - это тип, который представляет собой последовательность чисел.

```rust
let range = 1..10;
```

`Range` можно использовать в цикле `for`, `while` и `match`.

```rust
for x in 1..10 {
    println!("x = {}", x);
}

let x = 5;

match x {
    1..10 => println!("x = {}", x),
    _ => println!("x не входит в диапазон"),
}

while x in 1..10 {
    println!("x = {}", x);
}
```

И в остальных местах, где можно использовать переменную.

`Range` может также:

* Иметь шаг:

    ```rust
    let range = 1..10.step_by(2);
    ```

* Включать последнее число:

    ```rust
    let range = 1..=10;
    ```

* Иметь обратный порядок:

    ```rust
    let range = 10..1;
    ```

* Иметь обратный порядок с шагом:

    ```rust
    let range = 10..1.step_by(2);
    ```

* Иметь обратный порядок с шагом и включая последнее число:

    ```rust
    let range = 10..=1;
    ```

* Создавать очереди не только чисел, но и других типов:

    ```rust
    let range = 'a'..='z';
    ```

Подробнее о `Range` можно почитать в [документации](https://doc.rust-lang.org/std/ops/struct.Range.html).

## Функции

### Макросы

## Сноски

[^1]: `range` - это специальный тип, который позволяет получить последовательность чисел. Например, `0..10` - это последовательность чисел от `0` до `9`.

## Ссылки

* [The Rust Programming Language](https://doc.rust-lang.org/book/) - официальная документация по Rust
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - это книга, которая позволяет изучить язык Rust с помощью примеров кода.
* [Rust Playground](https://play.rust-lang.org/) - это онлайн-песочница, которая позволяет писать и запускать код на Rust.