
# Создание своего Telegram бота на Python 3.10

## Оглавление

1. [Установка зависимостей](#установка-зависимостей)
2. [Разбор примеров кода](#разбор-примеров-кода)
    1. [Приветстующий бот](#приветстующий-бот)
    2. [Обработка текстовых сообщений](#обработка-текстовых-сообщений)
    3. [Обработка неизвестных команд и ошибок](#обработка-неизвестных-команд-и-ошибок)
    4. [Использование клавиатуры](#использование-клавиатуры)
    5. [Inline клавиатура](#inline-клавиатура)
3. [Ссылки](#ссылки)
4. [Сноски](#сноски)

## Установка зависимостей

Бот будем разрабатывать на языке Python 3.10, поэтому для начала установим его.

### Windows

Для этого скачаем и установим [Python 3.10](https://www.python.org/downloads/). После запуска установщика, необходимо выбрать опцию "Add Python 3.x to PATH" (добавить Python 3.x в переменную PATH), чтобы можно было запускать Python из командной строки прсото набрав в ней `python` или `py`.

### Linux

В Linux Python 3 уже установлен, но его версия может быть ниже 3.10. Для проверки версии Python, введите в командной строке:

```bash
python --version
```

В случае, если версия Python ниже 3.10, то необходимо чтобы версия Python была не ниже 3.6. Если же версия Python ниже 3.6, то необходимо обновить Python. Для этого введите в командной строке:

```bash
sudo apt-get update
sudo apt-get install python3.10
```

В случае если Python 3 не был предустановлен, то необходимо его установить. Для этого установим Python 3.10 из репозитория:

```bash
sudo apt install python3.10
```

### Библиотеки

Мы будем использовать библиотеку [python-telegram-bot](https://github.com/python-telegram-bot/python-telegram-bot/) для создания бота. Для установки библиотеки и её зависимостей выполните команду:

```bash
pip install python-telegram-bot==20.0a2
```

Мы установили версию 20.0a2 (alpha), так как на момент написания статьи, она была последней версией библиотеки с новейшими функциями и большими изменениями.

## Разбор примеров кода

### Приветстующий бот

Рассмотрим пример кода, который создаст бота, который будет отвечать на команду `/start` и отправлять сообщение с приветствием.

```python
import logging
from telegram import Update
from telegram.ext import Application, CommandHandler, CallbackContext

# Токен бота из @BotFather
TOKEN = "YOUR_TOKEN"

# Включаем логирование, чтобы видеть сообщения об ошибках
logging.basicConfig(
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s', level=logging.INFO
)

logger = logging.getLogger(__name__)

async def start(update: Update, context: CallbackContext) -> None:
    """Отправляет сообщение с приветствием пользователю"""
    update.message.reply_text(f'Привет, {update.effective_user.first_name}!')

def main() -> None:
    """Запускает бота"""
    # Создаем экземпляр класса Application и передаем ему токен бота
    app = Application(TOKEN)

    # Добавляем обработчик команды /start
    app.add_handler(CommandHandler('start', start))

    # Запускаем бота
    app.run_polling()

if __name__ == '__main__':
    main()
```

На первых строках импортируются необходимые библиотеки и настраивается логирование. В переменную `TOKEN` записывается токен бота, который мы получили от `@BotFather` в самом Telegram.

Далее создается асинхронная функция[^1] `start`, которая будет вызываться при получении команды `/start`. В функцию передаются два аргумента: `update` и `context`. В `update` хранится информация о сообщении, которое отправил пользователь, а в `context` хранится информация о боте.

В функции мы отправляем сообщение с приветствием пользователю. Для этого используется метод `reply_text` класса `telegram.Update`, который принимает в качестве параметра текст сообщения.

```py
update.message.reply_text(f'Привет, {update.effective_user.first_name}!')
```

Далее создается функция `main`, которая будет запускать бота. В ней создается экземпляр класса `telegram.ext.Application` и передается токен бота. Далее добавляется обработчик команды `/start`, который будет вызывать функцию `start`. После этого запускается бот с помощью метода `run_polling`.

```py
def main() -> None:
    """Запускает бота"""
    # Создаем экземпляр класса Application и передаем ему токен бота
    app = Application(TOKEN)

    # Добавляем обработчик команды /start
    app.add_handler(CommandHandler('start', start))

    # Запускаем бота
    app.run_polling()
```

После этого мы запускаем бота, выполнив файл `main.py`:

```bash
python main.py
```

Заходим в Telegram и пишем боту `/start`. Бот должен ответить нам приветствием.

### Обработка текстовых сообщений

Давайте добавим обработку текстовых сообщений. Для этого нам нужно создать обработчик `MessageHandler` и передать ему фильтр `Filters.text`. В качестве второго аргумента передадим функцию `echo`, которая будет отправлять пользователю то же самое сообщение, которое он отправил боту.

```py
app.add_handler(MessageHandler(Filters.text, echo))
```

Функция `echo` достает из `update` текст сообщения и отправляет его пользователю.

```py
async def echo(update: Update, context: CallbackContext) -> None:
    """Отправляет сообщение пользователю"""
    update.message.reply_text(update.message.text)
```

### Обработка неизвестных команд и ошибок

#### Обработка неизвестных команд

Давайте добавим обработку неизвестных команд. Для этого нам нужно создать обработчик `MessageHandler` и передать ему фильтр[^2] `Filters.command`. В качестве второго аргумента передадим функцию `unknown`, которая будет отправлять пользователю сообщение об ошибке.

```py
app.add_handler(MessageHandler(Filters.command, unknown))
```

Функция `unknown` отправляет пользователю сообщение говорящее о том, что команда неизвестна.

```py
async def unknown(update: Update, context: CallbackContext) -> None:
    """Отправляет сообщение об ошибке"""
    update.message.reply_text('Неизвестная команда')
```

#### Обработка ошибок

Давайте добавим обработку ошибок. Для этого нам нужно создать обработчик `ErrorHandler` и передать ему функцию `error`, которая будет отправлять пользователю сообщение об ошибке.

```py
app.add_error_handler(error)
```

Функция `error` отправляет пользователю сообщение об ошибке.

```py
async def error(update: Update, context: CallbackContext) -> None:
    """Отправляет сообщение об ошибке"""
    update.message.reply_text('Ошибка')
```

### Использование клавиатуры

Давайте добавим клавиатуру, которая будет показываться пользователю при отправке сообщения боту. Для этого нам нужно создать клавиатуру и добавить в нее кнопки.

```py
reply_keyboard = [['/start', '/help']]
```

Далее нам нужно создать объект `ReplyKeyboardMarkup` и передать ему список кнопок.

```py
markup = ReplyKeyboardMarkup(reply_keyboard, one_time_keyboard=True)
```

После этого нам нужно добавить этот объект в `update.message.reply_text`.

```py
update.message.reply_text('Привет', reply_markup=markup)
```

В итоге получится следующий код.

```py
async def echo(update: Update, context: CallbackContext) -> None:
    """Отправляет сообщение пользователю"""
    reply_keyboard = [['/start', '/help']]
    markup = ReplyKeyboardMarkup(reply_keyboard, one_time_keyboard=True)
    update.message.reply_text(update.message.text, reply_markup=markup)
```

### Inline клавиатура

Давайте добавим inline клавиатуру, которая будет показываться пользователю при отправке сообщения боту. Для этого нам нужно создать inline клавиатуру и добавить в нее кнопки.

```py
reply_keyboard = [[InlineKeyboardButton('1', callback_data='1'),
                   InlineKeyboardButton('2', callback_data='2')]]
```

Далее нам нужно создать объект `InlineKeyboardMarkup` и передать ему список кнопок.

```py
markup = InlineKeyboardMarkup(reply_keyboard)
```

После этого нам нужно добавить этот объект в `update.message.reply_text`.

```py
update.message.reply_text('Привет', reply_markup=markup)
```

В итоге получится следующий код.

```py
async def echo(update: Update, context: CallbackContext) -> None:
    """Отправляет сообщение пользователю"""
    reply_keyboard = [[InlineKeyboardButton('1', callback_data='1'),
                       InlineKeyboardButton('2', callback_data='2')]]
    markup = InlineKeyboardMarkup(reply_keyboard)
    update.message.reply_text(update.message.text, reply_markup=markup)
```

Для обработки нажатий на кнопки нам нужно добавить обработчик `CallbackQueryHandler`.

```py
app.add_handler(CallbackQueryHandler(callback))
```

Функция `callback` обрабатывает нажатие на кнопку.

```py
async def callback(update: Update, context: CallbackContext) -> None:
    """Обрабатывает нажатие на кнопку"""
    update.callback_query.answer()
    update.callback_query.edit_message_text(text=f'Вы нажали на {update.callback_query.data}')
```

## Ссылки

* [telegram.ext](https://python-telegram-bot.readthedocs.io/en/stable/telegram.ext.html)
* [Примеры ботов из оффициальной документации](https://github.com/python-telegram-bot/python-telegram-bot/tree/master/examples)

## Сноски

[^1]: Асинхронные функции позволяют выполнять несколько задач одновременно. В данном случае, бот будет обрабатывать запросы от пользователей.
[^2]: Фильры из класса `Filters` позволяют фильтровать сообщения. В данном случае, фильтр `Filters.command` позволяет фильтровать сообщения, которые начинаются с символа `/`.
