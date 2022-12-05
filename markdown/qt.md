
# Описание основных возможностей Qt для С++

- [Описание основных возможностей Qt для С++](#описание-основных-возможностей-qt-для-с)
  - [Что такое Qt и где это можно использовать?](#что-такое-qt-и-где-это-можно-использовать)
  - [Пример простейшего приложения](#пример-простейшего-приложения)
  - [Приложение с формой входа в аккаунт](#приложение-с-формой-входа-в-аккаунт)

## Что такое Qt и где это можно использовать?

Qt - библиотека для создания эффективных современных GUI приложений на языке C++. Установить ее можно при помощи [установщика](https://www.qt.io/download) или [пакетного менеджера](https://wiki.qt.io/Install_Qt_5_on_Ubuntu) (для Linux). После установки Qt можно использовать в качестве библиотеки для создания GUI приложений, либо в качестве среды разработки (поставляется вместе с базовой библиотекой), в которой можно создавать проекты, использующие Qt.

Qt поддерживает различные платформы, включая Windows, Linux, macOS, Android, iOS, Sailfish OS, Windows Phone, Tizen, Raspberry Pi и многие другие. Подробнее о поддерживаемых платформах можно почитать [здесь](https://doc.qt.io/qt-5/supported-platforms.html).

## Пример простейшего приложения

Создадим простейшее приложение, которое будет выводить на экран окно с надписью "Hello, World!".

```cpp
#include <QApplication>
#include <QLabel>

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);                   // Создаем объект приложения
    QLabel *label = new QLabel("Hello, World!");    // Создаем объект надписи
    label->show();                                  // Отображаем наждпись на экране
    return app.exec();                              // Запускаем приложение
}
```

- `QApplication` - класс, который представляет собой приложение. Он создает окно приложения, обрабатывает события и управляет жизненным циклом приложения.
- `QLabel` - класс, который представляет собой надпись. Он может отображать текст и изображения (включая анимацию).

## Приложение с формой входа в аккаунт

Теперь создадим приложение, которое будет выводить на экран форму входа в аккаунт. Приложение будет состоять из двух полей ввода (для логина и пароля) и кнопки входа. При нажатии на кнопку входа будет всплывать уведомление с текстом "Вход выполнен".

Файл целиком:

```cpp
#include <QApplication>
#include <QLabel>
#include <QLineEdit>
#include <QPushButton>
#include <QVBoxLayout>

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);

    QLabel *loginLabel = new QLabel("Login:");  // Создаем объект надписи "Login:" 
    QLineEdit *loginEdit = new QLineEdit();     // Создаем объект поля ввода логина

    QLabel *passwordLabel = new QLabel("Password:");  // Создаем объект надписи "Password:"
    QLineEdit *passwordEdit = new QLineEdit();        // Создаем объект поля ввода пароля
    passwordEdit->setEchoMode(QLineEdit::Password);   // Устанавливаем режим ввода пароля
    
    // Вот несколько примеров с другими режимами:
    // passwordEdit->setEchoMode(QLineEdit::Normal);              // Стандартынй режим ввода текста
    // passwordEdit->setEchoMode(QLineEdit::NoEcho);              // Не показывать введенный текст
    // passwordEdit->setEchoMode(QLineEdit::PasswordEchoOnEdit);  // Показывать введенный текст, но скрывать его при потере фокуса

    QPushButton *loginButton = new QPushButton("Login"); // Создаем объект кнопки входа

    // Создаем объект надписи, которая будет отображать уведомление
    QLabel *notificationLabel = new QLabel();
    notificationLabel->setAlignment(Qt::AlignCenter); // Выравниваем текст по центру

    // Создаем объект вертикального контейнера, который будет содержать все элементы
    QVBoxLayout *layout = new QVBoxLayout();
    layout->addWidget(loginLabel);      // Добавляем надпись "Login:"
    layout->addWidget(loginEdit);       // Добавляем поле ввода логина
    layout->addWidget(passwordLabel);   // Добавляем надпись "Password:"
    layout->addWidget(passwordEdit);    // Добавляем поле ввода пароля
    layout->addWidget(loginButton);     // Добавляем кнопку входа
    layout->addWidget(notificationLabel); // Добавляем надпись для уведомления

    // Создаем объект окна, который будет содержать вертикальный контейнер
    QWidget window;
    window.setLayout(layout); // Устанавливаем вертикальный контейнер в качестве менеджера компоновки окна

    // Подключаем сигнал нажатия на кнопку входа к слоту, который будет отображать уведомление
    QObject::connect(loginButton, &QPushButton::clicked, [&](){
        notificationLabel->setText("Login: " + loginEdit->text() + "\nPassword: " + passwordEdit->text());
    });

    window.show();

    return app.exec();
}
```

А теперь разберем по кускам:

Создаем объекты надписей и полей ввода с помощью конструктора класса `QLabel` и `QLineEdit`.

```cpp
QLabel *loginLabel = new QLabel("Login:");
QLineEdit *loginEdit = new QLineEdit();
```

Устанавливаем режим ввода пароля для поля ввода пароля.

```cpp
passwordEdit->setEchoMode(QLineEdit::Password);
```

Создаем объект кнопки входа с помощью конструктора класса `QPushButton`. В качестве параметра передаем текст, который будет отображаться на кнопке. В данном случае это будет `"Login"`. Также создаем объект надписи, которая будет отображать уведомление и центрируем текст в ней с помощью метода `setAlignment` и флага `Qt::AlignCenter`.

```cpp
QPushButton *loginButton = new QPushButton("Login");
QLabel *notificationLabel = new QLabel();
notificationLabel->setAlignment(Qt::AlignCenter);
```

Создаем объект вертикального контейнера, который будет содержать все элементы. Для этого используем конструктор класса `QVBoxLayout`. Контейнеры в QT позволяют по разному размещать виджеты на экране. В данном случае мы используем вертикальный контейнер, который будет размещать виджеты друг под другом. 

```cpp
QVBoxLayout *layout = new QVBoxLayout();
```
