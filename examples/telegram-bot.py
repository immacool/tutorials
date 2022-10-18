import logging
from telegram import Update
from telegram.ext import Updater, CommandHandler, CallbackContext

# Включаем логирование, чтобы видеть сообщения об ошибках
logging.basicConfig(
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s', level=logging.INFO
)

logger = logging.getLogger(__name__)

def start(update: Update, context: CallbackContext) -> None:
    """Отправляет сообщение с приветствием пользователю"""
    update.message.reply_text('Привет!')

def main() -> None:
    """Запускает бота"""
    # Создаем Updater и передаем ему токен бота. Токен бота выдается @BotFather
    updater = Updater("TOKEN")

    # Получаем диспетчер из updater
    dispatcher = updater.dispatcher

    # На команду /start вызываем функцию start
    dispatcher.add_handler(CommandHandler("start", start))

    # Начинаем поиск обновлений
    updater.start_polling()

    # Останавливаем бота, если были нажаты Ctrl + C
    updater.idle()

if __name__ == '__main__':
    main()