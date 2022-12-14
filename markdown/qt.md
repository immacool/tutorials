
# Описание основных возможностей Qt для С++

- [Описание основных возможностей Qt для С++](#описание-основных-возможностей-qt-для-с)
  - [Что такое Qt и где это можно использовать?](#что-такое-qt-и-где-это-можно-использовать)
  - [Пример простейшего приложения](#пример-простейшего-приложения)
    - [Сборка и запуск приложения](#сборка-и-запуск-приложения)
  - [Приложение с формой входа в аккаунт](#приложение-с-формой-входа-в-аккаунт)
  - [Приложение с кастомным главным окном](#приложение-с-кастомным-главным-окном)

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

### Сборка и запуск приложения

Для компиляции необходим файл с разширением `.pro`. В нем указывается название проекта, используемые библиотеки и исходные файлы. При помощи команды `qmake` можно создать файл с расширением `.pro` с конфигурацией по умолчанию. Для этого воспользуемся командой `qmake -project`:

```bash
qmake -project
```

И в результате будет создан файл `hello.pro`:

```bash
######################################################################
# Automatically generated by qmake (3.1) Mon Dec 5 00:11:56 2022
######################################################################

TEMPLATE = app
TARGET = test
INCLUDEPATH += .

# You can make your code fail to compile if you use deprecated APIs.
# In order to do so, uncomment the following line.
# Please consult the documentation of the deprecated API in order to know
# how to port your code away from it.
# You can also select to disable deprecated APIs only up to a certain version of Qt.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

# Input
SOURCES += hello.cpp
```

Но нам нужно будет добавить в файл `hello.pro` еще одну строку:

```bash
QT += widgets
```

Так мы подключим библиотеку `widgets`, которая необходима для работы с виджетами.

Теперь можно скомпилировать и запустить приложение. Для этого воспользуемся командой `qmake`:

```bash
qmake hello.pro
```

После этого будет создан файл `Makefile`, который можно использовать для компиляции и запуска приложения:

```bash
make
./hello
```

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
    layout->addWidget(loginLabel);        // Добавляем надпись "Login:"
    layout->addWidget(loginEdit);         // Добавляем поле ввода логина
    layout->addWidget(passwordLabel);     // Добавляем надпись "Password:"
    layout->addWidget(passwordEdit);      // Добавляем поле ввода пароля
    layout->addWidget(loginButton);       // Добавляем кнопку входа
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

## Приложение с кастомным главным окном

Создадим приложение Qt с окном класса `QMainWindow`. Таким образом мы сможем контролировать события происходящие в окне и его конфигруацию. Рассмотрим пример:

Для начала необходимо создать следующую структуру проекта:

```yaml
- <project>
  - src
    - main.cpp
    - mainwindow.cpp
    - mainwindow.h
  - <project>.pro
```

Заголовочный файл нужен для того чтобы корректно работать с собственными слотами[^1] и событиями, а так же с родными для окна. В нем мы объявляем класс `MainWindow` и наследуем его от `QMainWindow`. Также объявляем слоты и сигналы, которые будут использоваться в нашем приложении.

```cpp
#ifndef MAINWINDOW_H // Защита от повторного подключения заголовочного файла
#define MAINWINDOW_H

#include <QMainWindow>

class MainWindow : public QMainWindow // Наследуем класс от QMainWindow
{
    Q_OBJECT // Объявляем макрос Q_OBJECT для использования слотов и сигналов
             // Он позволяет нам использовать ключевое слово signals и slots в нашем классе

public:
    MainWindow(QWidget *parent = nullptr); // Конструктор и деструктор
    ~MainWindow();                         // класса

private slots:
    void onButtonClicked(); // Объявляем слот, который будет вызываться при нажатии на кнопку

private:
    QPushButton *button; // Объявляем указатель на кнопку
};

#endif // MAINWINDOW_H
```

Мы создали заголовочный файл где объявили класс `MainWindow` и создали поле для кнопки вместе со слотом нажатия на нее. Теперь необходимо реализовать[^2] этот класс в файле `mainwindow.cpp`. 

```cpp
#include "mainwindow.h" // Подключаем заголовочный файл с обявлением класса
#include <QPushButton>  // Класс для создания кнопки
#include <QMessageBox>  // Класс для вызова диалогового окна

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) // Конструктор класса
{
    button = new QPushButton("Click me!", this);                       // Создаем кнопку и передаем родителя
    button->setGeometry(10, 10, 100, 30);                              // Задаем размеры кнопки
    connect(button, SIGNAL(clicked()), this, SLOT(onButtonClicked())); // Устанавливаем соединение между сигналом нажатия на кнопку и слотом onButtonClicked()
}

MainWindow::~MainWindow() // Деструктор класса
{
    delete button; // Удаляем кнопку
}

void MainWindow::onButtonClicked() // Реализация слота
{
    QMessageBox::information(this, "Hello!", "Hello, World!"); // Выводим сообщение
}
```

Строка `connect(button, SIGNAL(clicked()), this, SLOT(onButtonClicked()));` устанавливает соединение между сигналом нажатия на кнопку и слотом `onButtonClicked()`. Сигналы и слоты позволяют нам обрабатывать события в нашем приложении. В данном случае мы подключаем сигнал нажатия на кнопку к слоту `onButtonClicked()`. Теперь при нажатии на кнопку будет вызываться слот `onButtonClicked()`.

Теперь необходимо создать файл `main.cpp` в котором будет находиться точка входа в программу.

```cpp
#include <QApplication> // Класс для создания приложения
#include "mainwindow.h" // Подключаем заголовочный файл с обявлением класса

int main(int argc, char *argv[]) // Точка входа в программу
{
    QApplication a(argc, argv); // Создаем приложение
    MainWindow w;               // Создаем объект класса MainWindow
    w.show();                   // Показываем окно
    return a.exec();            // Запускаем приложение
}
```

В данном случае мы создаем объект класса `MainWindow` и показываем его. Теперь необходимо создать файл `main.pro` в котором будет находиться информация о проекте.

```js
QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

TARGET = main
TEMPLATE = app

SOURCES += \
        main.cpp \
        mainwindow.cpp

HEADERS  += \
        mainwindow.h
```

Теперь соберем проект и запустим его:

```bash
qmake main.pro
make
./main
```