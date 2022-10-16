
# Азы Kotlin

## Работа с переменными

Оглавление:

* [Инициализация переменных и их виды](#Инициализация переменных и их виды)
* [Стандартные типы данных](#Стандартные типы данных)
* [Преобразование типов](#Преобразование типов)
* [Строки](#Строки)
* [Массивы](#Массивы)

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

* `Boolean` - логический тип данных, принимает значения `true` или `false`.
* `Byte` - целочисленный тип данных, принимает значения от `-128` до `127`.
* `Short` - целочисленный тип данных, принимает значения от `-32768` до `32767`.
* `Int` - целочисленный тип данных, принимает значения от `-2147483648` до `2147483647`.
* `Long` - целочисленный тип данных, принимает значения от `-9223372036854775808` до `9223372036854775807`.
* `Float` - вещественный тип данных, принимает значения от `1.4e-45` до `3.4028235e+38`.
* `Double` - вещественный тип данных, принимает значения от `4.9e-324` до `1.7976931348623157e+308`.
* `Char` - символьный тип данных, принимает значения от `'\u0000'` до `'\uffff'`.

Объектами первого порядка являются все типы данных, кроме `Boolean`, `Char`, `Float` и `Double`. Таким образом, переменные типа `Int` могут быть присвоены переменным типа `Long`, а переменные типа `Long` могут быть присвоены переменным типа `Int`. То же самое касается типов `Byte` и `Short`.

```kotlin
val a: Int = 1
val b: Long = a

val e: Byte = 1
val f: Short = e

val g: Short = 1
val h: Int = g
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

* `\t` - символ табуляции.
* `\b` - символ возврата курсора.
* `\n` - символ перевода строки.
* `\r` - символ возврата каретки.
* `\'` - символ одинарной кавычки.
* `\"` - символ двойной кавычки.
* `\\` - символ обратной косой черты.

```kotlin
val a = "Hello\tWorld"
val b = "Hello\nWorld"
```

#### Строковые функции

В Kotlin есть несколько полезных функций для работы со строками.

* `length` - длина строки.
* `get(index)` - получение символа по индексу.
* `substring(startIndex, endIndex)` - получение подстроки.
* `contains(other)` - проверка на вхождение подстроки.
* `startsWith(prefix)` - проверка на начало строки.
* `endsWith(suffix)` - проверка на конец строки.
* `replace(oldValue, newValue)` - замена подстроки.

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