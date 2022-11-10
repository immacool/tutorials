
# Азы Kotlin

Оглавление:

- [Азы Kotlin](#азы-kotlin)
  - [Работа с переменными](#работа-с-переменными)
    - [Инициализация переменных и их виды](#инициализация-переменных-и-их-виды)
    - [Стандартные типы данных](#стандартные-типы-данных)
      - [Пара слов о численных типах](#пара-слов-о-численных-типах)
    - [Преобразование типов](#преобразование-типов)
    - [Строки](#строки)
      - [Строковые шаблоны](#строковые-шаблоны)
      - [Строковые литералы](#строковые-литералы)
      - [Строковые функции](#строковые-функции)
    - [Массивы](#массивы)
    - [Коллекции](#коллекции)
      - [Список](#список)
      - [Множество](#множество)
      - [Словарь](#словарь)
      - [Последовательность](#последовательность)
    - [Функции](#функции)
      - [Объявление функции](#объявление-функции)
      - [Анонимные функции](#анонимные-функции)
      - [Лямбда-выражения](#лямбда-выражения)
      - [Переменное число аргументов](#переменное-число-аргументов)
  - [ООП](#ооп)
    - [Создание класса](#создание-класса)
    - [Переопределение методов](#переопределение-методов)
    - [Конструкторы](#конструкторы)
    - [Перегрузка конструкторов](#перегрузка-конструкторов)
    - [Инициализация](#инициализация)
    - [Наследование](#наследование)
    - [Абстрактные классы](#абстрактные-классы)
    - [Интерфейсы](#интерфейсы)
    - [Делегирование](#делегирование)
  - [Ссылки](#ссылки)
  - [Сноски](#сноски)

## Работа с переменными

Прежде всего, знакомство стоит начать с самого простого - наименьшей единицы языка - переменной.

В главах, идущих далее будут разбираться основные возможности языка, а также приведены примеры кода, которые можно запустить в интерактивном режиме на сайте [try.kotlinlang.org](https://try.kotlinlang.org/).

### Инициализация переменных и их виды

В Kotlin существует два типа переменных: `var` и `val`. Переменная `var` может быть изменена (является иммутабельной), а переменная `val` - нет (является мутабельной). Переменные `var` и `val` могут быть инициализированы при объявлении, а также в любой момент в коде.

```kotlin
var a = 1
val b = 2
```

Так же существует тип `const`, который является константой и не может быть изменен во время выполнения программы.

```kotlin
const val c = 3
```

### Стандартные типы данных

В Kotlin существует 8 стандартных типов данных:

- `Boolean` - логический тип данных, принимает значения `true` или `false`.
- `Byte` - целочисленный тип данных, принимает значения от `-128` до `127`.
- `Short` - целочисленный тип данных, принимает значения от `-32768` до `32767`.
- `Int` - целочисленный тип данных, принимает значения от `-2147483648` до `2147483647`.
- `Long` - целочисленный тип данных, принимает значения от `-9223372036854775808` до `9223372036854775807`.
- `Float` - вещественный тип данных, принимает значения от `1.4e-45` до `3.4028235e+38`.
- `Double` - вещественный тип данных, принимает значения от `4.9e-324` до `1.7976931348623157e+308`.
- `Char` - символьный тип данных, принимает значения от `'\u0000'` до `'\uffff'`.

Объектами первого порядка являются все типы данных, кроме `Boolean`, `Char`, `Float` и `Double`. Таким образом, переменные типа `Int` могут быть присвоены переменным типа `Long`, а переменные типа `Long` могут быть присвоены переменным типа `Int`. То же самое касается типов `Byte` и `Short`.

```kotlin
val a: Int = 1
val b: Long = a

val e: Byte = 1
val f: Short = e

val g: Short = 1
val h: Int = g
```

#### Пара слов о численных типах

В Kotlin существует два[^1] типа чисел: `Int` и `Double`. При инициализации переменной типа `Double` необходимо указывать значение после запятой, иначе она будет инициализирована как `Int`.

```kotlin
val a = 1
val b = 1.0
```

Так же существует тип `Float`, который является вещественным числом с плавающей точкой. Так же как и `Double`, он необходимо инициализировать после запятой.

```kotlin
val a: Float = 1.0
```

### Преобразование типов

В Kotlin существует несколько способов преобразования типов. Например можно использовать функцию `toByte()`, `toShort()`, `toInt()`, `toLong()`, `toFloat()`, `toDouble()`, `toChar()`.

```kotlin
val a: Int = 1
val b: Long = a.toLong()
```

Также можно использовать оператор `as`, который преобразует тип данных, если это возможно.

```kotlin
val a: Int = 1
val b: Long = a as Long
```

### Строки

Строки в Kotlin являются неизменяемыми, то есть их нельзя изменить после создания. Так же язык поддерживает многострочные строки, которые начинаются и заканчиваются с `"""`.

```kotlin
val a = "Hello"
val b = "World"
val c = a + b // Операця конкатенации строк
val e = """Hello
World"""
```

#### Строковые шаблоны

В Kotlin есть возможность использовать строковые шаблоны, которые позволяют вставлять значения переменных в строку. Для этого используется символ `$`.

```kotlin
val a = 1
val b = "a is $a"
```

Также можно использовать выражения внутри строковых шаблонов.

```kotlin
val a = 1
val b = "a is ${a + 2}"
```

#### Строковые литералы

В Kotlin есть возможность использовать специальные символы в строках. Для этого используются специальные последовательности символов, которые начинаются с `\`.

- `\t` - символ табуляции.
- `\b` - символ возврата курсора.
- `\n` - символ перевода строки.
- `\r` - символ возврата каретки.
- `\'` - символ одинарной кавычки.
- `\"` - символ двойной кавычки.
- `\\` - символ обратной косой черты.

```kotlin
val a = "Hello\tWorld"
val b = "Hello\nWorld"
```

#### Строковые функции

В Kotlin есть несколько полезных функций для работы со строками.

- `length` - длина строки.
- `get(index)` - получение символа по индексу.
- `substring(startIndex, endIndex)` - получение подстроки.
- `contains(other)` - проверка на вхождение подстроки.
- `startsWith(prefix)` - проверка на начало строки.
- `endsWith(suffix)` - проверка на конец строки.
- `replace(oldValue, newValue)` - замена подстроки.

```kotlin
val a = "Hello World"
a.length // 11
a.get(0) // H
a.substring(0, 5) // Hello
a.contains("Hello") // true
a.startsWith("Hello") // true
a.endsWith("World") // true
a.replace("Hello", "Goodbye") // Goodbye World
```

### Массивы

Массивы в Kotlin являются неизменяемыми, то есть их нельзя изменить после создания. Для создания массива используется функция `arrayOf()`.

```kotlin
val a = arrayOf(1, 2, 3)
```

Для доступа к элементам массива используется индексация.

```kotlin
val a = arrayOf(1, 2, 3)
a[0] // 1
a[1] // 2
a[2] // 3
```

Стандаартные операции над массивами:

- `size` - размер массива.
- `contains(element)` - проверка на вхождение элемента.
- `indexOf(element)` - получение индекса элемента.
- `lastIndexOf(element)` - получение индекса последнего вхождения элемента.
- `first()` - первый элемент массива.
- `last()` - последний элемент массива.

```kotlin
val a = arrayOf(1, 2, 3)
a.size // 3
a.contains(2) // true
a.indexOf(2) // 1
a.lastIndexOf(2) // 1
a.first() // 1
a.last() // 3
```

### Коллекции

В Kotlin есть несколько типов коллекций:

- `List` - список, упорядоченная коллекция элементов. Отличается от массива тем, что его размер можно изменять.
- `Set` - множество, неупорядоченная коллекция элементов, в которой каждый элемент встречается не более одного раза.
- `Map` - словарь, неупорядоченная коллекция пар ключ-значение.

#### Список

Список в Kotlin представляет собой неизменяемый список элементов. Для создания списка используется функция `listOf()`.

```kotlin
val a = listOf(1, 2, 3)
```

Для доступа к элементам списка используется индексация.
Стандартные операции над списками см. в разделе про массивы, так как они идентичны.

Пример использования списка:

```kotlin
val a = listOf(1, 2, 3)
val b = a + 4 // [1, 2, 3, 4]
val c = a - 2 // [1, 3]
val d = a - listOf(1, 2) // [3]
```

Операции, уникальные для списков:

- `slice(indices)` - получение подсписка по индексам.
- `slice(range)` - получение подсписка по диапазону индексов.
- `subList(fromIndex, toIndex)` - получение подсписка по диапазону индексов.

```kotlin
val a = listOf(1, 2, 3)
a.slice(0..1) // [1, 2]
a.slice(listOf(0, 2)) // [1, 3]
a.subList(0, 2) // [1, 2]
```

#### Множество

Множество в Kotlin представляет собой неизменяемое множество элементов. Для создания множества используется функция `setOf()`.

```kotlin
val a = setOf(1, 2, 3)
```

Для доступа к элементам множества используется индексация.
Стандартные операции над множествами см. в разделе про массивы, так как они идентичны.

Пример использования множества:

```kotlin
val a = setOf(1, 2, 3)
val b = a + 4 // [1, 2, 3, 4]
val c = a - 2 // [1, 3]
val d = a - setOf(1, 2) // [3]
```

Операции, уникальные для множеств:

- `intersect(other)` - пересечение множеств.
- `subtract(other)` - разность множеств.
- `union(other)` - объединение множеств.

```kotlin
val a = setOf(1, 2, 3)
val b = setOf(2, 3, 4)
a.intersect(b) // [2, 3]
a.subtract(b) // [1]
a.union(b) // [1, 2, 3, 4]
```

#### Словарь

Словарь в Kotlin представляет собой неизменяемый словарь. Для создания словаря используется функция `mapOf()`.

```kotlin
val a = mapOf("one" to 1, "two" to 2, "three" to 3)
```

Для доступа к элементам словаря используется ключ.

```kotlin
val a = mapOf("one" to 1, "two" to 2, "three" to 3)
a["one"] // 1
```

Стандартные операции над словарями:

- `+` - объединение словарей.
- `-` - удаление элементов из словаря.

```kotlin
val a = mapOf("one" to 1, "two" to 2, "three" to 3)
val b = a + ("four" to 4) // {"one" to 1, "two" to 2, "three" to 3, "four" to 4}
val c = a - "two" // {"one" to 1, "three" to 3}
```

Операции, уникальные для словарей:

- `mapKeys(transform)` - преобразование ключей.
- `mapValues(transform)` - преобразование значений.

Далее используются анонимные функции, которые будут рассмотрены в разделе [Функции](#функции).

```kotlin
val a = mapOf("one" to 1, "two" to 2, "three" to 3)
a.mapKeys { it.key.toUpperCase() } // {"ONE" to 1, "TWO" to 2, "THREE" to 3}
a.mapValues { it.value * 2 } // {"one" to 2, "two" to 4, "three" to 6}
```

#### Последовательность

Последовательность в Kotlin представляет собой неизменяемую последовательность элементов. Для создания последовательности используется функция `sequenceOf()`.

```kotlin
val a = sequenceOf(1, 2, 3)
```

Для доступа к элементам последовательности используется индексация.

Операции уникальные для последовательностей:

- `filter(predicate)` - фильтрация элементов.
- `map(transform)` - преобразование элементов.
- `take(n)` - взять первые n элементов.
- `drop(n)` - отбросить первые n элементов.

Далее используются анонимные функции, которые будут рассмотрены в разделе [Функции](#функции).

```kotlin
val a = sequenceOf(1, 2, 3, 4, 5)
a.filter { it % 2 == 0 } // [2, 4]
a.map { it * 2 } // [2, 4, 6, 8, 10]
a.take(2) // [1, 2]
a.drop(2) // [3, 4, 5]
```

### Функции

Функции в Kotlin являются объектами первого класса. Они могут быть переданы в качестве аргументов, возвращены из других функций и храниться в переменных.

#### Объявление функции

Объявление функции начинается с ключевого слова `fun`, за которым следует имя функции, список аргументов и тип возвращаемого значения.

```kotlin
fun sum(a: Int, b: Int): Int {
    return a + b
}
```

Тип возвращаемого значения может быть опущен, если функция ничего не возвращает.

```kotlin
fun printSum(a: Int, b: Int) {
    println("Сумма равна ${a + b}")
}
```

Аргументы функции могут иметь значения по умолчанию.

```kotlin
fun printSum(a: Int = 2, b: Int = 1) {
    println("Сумма равна ${a + b}")
}
```

Вызов функции производится по имени функции, за которым следует список аргументов.

```kotlin
sum(1, 2) // 3
printSum(1, 2) // Сумма равна 3
printSum() // Сумма равна 3
```

Так же в языке есть именованные аргументы.

```kotlin
printSum(b = 2) // Сумма равна 4
```

#### Анонимные функции

Анонимные функции - это функции, которые не имеют имени. Вместо имени используется ключевое слово `fun`, а вместо тела функции используется выражение.

```kotlin
val sum = fun(a: Int, b: Int): Int {
    return a + b
}
```

#### Лямбда-выражения

Лямбда-выражения - это анонимные функции, которые могут быть переданы в качестве аргументов другим функциям, или присвоены переменной.

```kotlin
val sum = { a: Int, b: Int -> a + b }
```

Если лямбда-выражение состоит из одного выражения, то фигурные скобки можно опустить.

```kotlin
val sum = { a: Int, b: Int -> a + b }
```

Лямбда-выражения могут быть использованы в качесте предикатов в функциях обхода коллекций.

Вот пример использования в функции `filter`.

```kotlin
val numbers = listOf(1, 2, 3, 4, 5)
val evenNumbers = numbers.filter { it % 2 == 0 }
println(evenNumbers) // [2, 4]
```

В подобных функциях `it` - это аргумент, который передается в лямбда-выражение. В данном случае `it` - это элемент итерируемой коллекции `numbers`.

#### Переменное число аргументов

Функции могут принимать переменное число аргументов.

```kotlin
fun printAll(vararg messages: String) {
    for (m in messages) println(m)
}
```

В вызове функции можно передать N число аргументов.

```kotlin
printAll("Hello", "Hallo", "Salut", "Hola", "你好")
```

Вывод:

```txt
Hello
Hallo
Salut
Hola
你好
```

## ООП

### Создание класса

Класс - это шаблон, по которому создаются объекты. Классы в Kotlin объявляются с помощью ключевого слова `class`.

```kotlin
class Person
```

Класс может иметь различные параметры в виде свойств, методов, полей и т.д.

```kotlin
class Person {
    // поля
    var name: String = ""
    var age: Int = 0
    
    // свойство
    val isAdult: Boolean
        get() = age >= 18

    // метод
    fun greet() {
        println("Hello, my name is $name")
    }
}
```

Свойства класса объявляются с помощью ключевого слова `var` или `val`.

```kotlin
var name: String = ""
val age: Int = 0
```

Классы могут быть инициализированы с помощью конструктора.

```kotlin
class Person(name: String, age: Int) {
    var name = name
    var age = age
    fun printInfo() {
        println("Имя: $name, возраст: $age")
    }
}
```

`class Person(name: String, age: Int)` - это конструктор класса. Конструкторы могут иметь параметры, которые будут переданы при создании объекта.

```kotlin
val person = Person("John", 20)
```

### Переопределение методов

Методы класса можно переопределить.

```kotlin
class Person(name: String, age: Int) {
    var name = name
    var age = age
    fun printInfo() {
        println("Имя: $name, возраст: $age")
    }
    override fun toString(): String {
        return "Person(name='$name', age=$age)"
    }
}
```

Так же можно переопределить методы операторов.

Вот пример переопределения `==` и `+`

```kotlin
class Person(name: String, age: Int) {
    var name = name
    var age = age
    fun printInfo() {
        println("Имя: $name, возраст: $age")
    }
    override fun toString(): String {
        return "Person(name='$name', age=$age)"
    }

    override fun hashCode(): Int {
        var result = name.hashCode()
        result = 31 * result + age
        return result
    }

    override fun equals(other: Person): Boolean {
        return this.hashCode() == other.hashCode()
    }

    operator fun plus(other: Person): Person {
        return Person(name + other.name, age + other.age)
    }
}

fun main() {
    val p1 = Person("Иван", 20)
    val p2 = Person("Иван", 20)
    println(p1 == p2) // true
    println(p1 + p2) // Person(name='ИванИван', age=40)
}
```

### Конструкторы

Конструкторы класса объявляются с помощью ключевого слова `constructor`.

```kotlin
class Person constructor(name: String, age: Int) {
    var name = name
    var age = age
    fun printInfo() {
        println("Имя: $name, возраст: $age")
    }
}
```

Если ключевое слово `constructor` не используется, то можно опустить его.

```kotlin
class Person(name: String, age: Int) {
    var name = name
    var age = age
    fun printInfo() {
        println("Имя: $name, возраст: $age")
    }
}
```

Конструкторы могут иметь параметры по умолчанию, так же как и обычные функции или методы.

```kotlin
class Person(name: String, age: Int = 0) {
    var name = name
    var age = age
    fun printInfo() {
        println("Имя: $name, возраст: $age")
    }
}
```


### Перегрузка конструкторов

Конструкторы можно перегружать.

```kotlin
class Person {
    var name: String
    var age: Int

    constructor(name: String) {
        this.name = name
        this.age = 0
    }

    constructor(name: String, age: Int) {
        this.name = name
        this.age = age
    }
}
```

### Инициализация

Класс может содержать блок инициализации. Блок инициализации выполняется сразу после создания объекта.

```kotlin
class Person {
    var name: String
    var age: Int

    init {
        name = "John"
        age = 20
    }
}
```

### Наследование

Классы могут наследовать друг друга с помощью ключевого слова `open`, обозначающего открытый класс, т.е. класс, который может быть наследован.

```kotlin
open class Person {
    var name: String = ""
    var age: Int = 0
}

class Student: Person() {
    var university: String = ""
}
```

Классы наследуются с помощью оператора `:`

```kotlin
class Student: Person()
```

Если класс наследуется от другого класса, то он может переопределить свойства и методы родительского класса.

```kotlin
open class Person {
    var name: String = ""
    var age: Int = 0
}

class Student: Person() {
    var university: String = ""
    override fun toString(): String {
        return "Student(name='$name', age=$age, university='$university')"
    }
}

fun main() {
    val student = Student()
    student.name = "Иван"
    student.age = 20
    student.university = "МГУ"
    println(student)
}
```

### Абстрактные классы

Абстрактные классы могут содержать абстрактные методы, которые должны быть переопределены в наследнике.

```kotlin
abstract class Person {
    var name: String = ""
    var age: Int = 0
    abstract fun printInfo()
}

class Student: Person() {
    var university: String = ""
    override fun printInfo() {
        println("Имя: $name, возраст: $age, университет: $university")
    }
}

fun main() {
    val student = Student()
    student.name = "Иван"
    student.age = 20
    student.university = "МГУ"
    student.printInfo()
}
```

### Интерфейсы

Интерфейсы можно реализовать с помощью ключевого слова `interface`.

```kotlin
interface Person {
    var name: String
    var age: Int
    fun printInfo()
}

class Student: Person {
    override var name: String = ""
    override var age: Int = 0
    var university: String = ""
    override fun printInfo() {
        println("Имя: $name, возраст: $age, университет: $university")
    }
}

fun main() {
    val student = Student()
    student.name = "Иван"
    student.age = 20
    student.university = "МГУ"
    student.printInfo()
}
```

Интерфейсы могут наследовать другие интерфейсы.

```kotlin
interface Person {
    var name: String
    var age: Int
    fun printInfo()
}

interface Student: Person {
    var university: String
}

class StudentImpl: Student {
    override var name: String = ""
    override var age: Int = 0
    override var university: String = ""
    override fun printInfo() {
        println("Имя: $name, возраст: $age, университет: $university")
    }
}

fun main() {
    val student = StudentImpl()
    student.name = "Иван"
    student.age = 20
    student.university = "МГУ"
    student.printInfo()
}
```

### Делегирование

Делегирование позволяет реализовать интерфейс с помощью другого объекта.

```kotlin
interface Person {
    var name: String
    var age: Int
    fun printInfo()
}

class StudentImpl: Person {
    override var name: String = ""
    override var age: Int = 0
    override fun printInfo() {
        println("Имя: $name, возраст: $age")
    }
}

class Student(person: Person): Person by person

fun main() {
    val studentImpl = StudentImpl()
    studentImpl.name = "Иван"
    studentImpl.age = 20
    val student = Student(studentImpl)
    student.printInfo()
}
```

## Ссылки

- [Оффициальная документация](https://kotlinlang.org/docs/reference/) (kotlinlang.org)
- [Kotlin Koans](https://play.kotlinlang.org/koans/overview) (play.kotlinlang.org)
- [Wiki](https://ru.wikipedia.org/wiki/Kotlin) (ru.wikipedia.org)

## Сноски
