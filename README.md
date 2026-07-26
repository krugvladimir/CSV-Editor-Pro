📊 CSV Editor Pro — табличный редактор CSV
Мощный консольный редактор CSV-файлов с поддержкой табличного просмотра, редактирования, сортировки, фильтрации и экспорта.
Реализован на 7 языках программирования для демонстрации работы с данными и текстовыми интерфейсами.

https://img.shields.io/github/repo-size/yourname/csveditor
https://img.shields.io/github/stars/yourname/csveditor?style=social
https://img.shields.io/badge/License-MIT-blue.svg

🧠 Концепция
CSV Editor Pro — это интерактивный редактор для работы с CSV-файлами. Он позволяет:

✅ Загружать CSV-файлы с разделителем (запятая, точка с запятой, табуляция).

✅ Просматривать данные в табличном виде с выравниванием по столбцам.

✅ Добавлять, удалять и редактировать строки и ячейки.

✅ Сортировать по любому столбцу (по возрастанию/убыванию).

✅ Фильтровать строки по значению в столбце.

✅ Сохранять изменения в текущий файл или как новый файл.

✅ Автоопределение разделителя.

✅ Цветной вывод (в большинстве версий) для удобства.

✅ История команд (в некоторых версиях).

🚀 Как запустить
Каждая версия является консольным приложением. Инструкции по установке и запуску:

Python
bash
python csv_editor_python.py
C++
bash
g++ -std=c++17 csv_editor_cpp.cpp -o csv_editor
./csv_editor
Java
bash
javac CSVEditorJava.java && java CSVEditorJava
C# (.NET Core)
bash
dotnet run
Go
bash
go mod init csveditor
go run csv_editor_go.go
Rust
bash
cargo new csv_editor
cd csv_editor
# Добавьте зависимости в Cargo.toml (см. код)
cargo run
JavaScript (Node.js)
bash
npm install
node csv_editor_js.js
🧩 Пример сессии
text
$ csv_editor
📊 CSV Editor Pro v2.0
> load data.csv
✅ Загружено 5 строк, 3 столбца

> list
  #  Name      Age  City
  1  Alice     25   New York
  2  Bob       30   London
  3  Charlie   28   Paris
  4  David     22   Berlin
  5  Eve       35   Madrid

> sort Age desc
  #  Name      Age  City
  1  Eve       35   Madrid
  2  Bob       30   London
  3  Charlie   28   Paris
  4  Alice     25   New York
  5  David     22   Berlin

> filter City London
  #  Name      Age  City
  1  Bob       30   London

> add
Введите значения через запятую: Frank,40,Tokyo
✅ Строка добавлена

> save
✅ Сохранено в data.csv
> exit
📦 Содержимое репозитория
Файл	Язык	Особенности
csv_editor_python.py	Python	colorama, in-memory, команды
csv_editor_cpp.cpp	C++	цветной вывод (ANSI), стандартная библиотека
CSVEditorJava.java	Java	консоль, цветной вывод (ANSI)
CSVEditorCSharp.cs	C#	цветной вывод, команды
csv_editor_go.go	Go	цветной вывод, горутины (необязательно)
csv_editor_rs.rs	Rust	termion, цветной вывод
csv_editor_js.js	JavaScript	readline, цветной вывод (chalk)
🔮 Расширенные функции
Автоопределение разделителя — запятая, точка с запятой, табуляция.

Экспорт в JSON (в некоторых версиях).

История команд (в Python и Rust).

Пакетная обработка (в планах).

📜 Лицензия
MIT — свободно используйте, модифицируйте и распространяйте.

🤝 Вклад
Приветствуются пул-реквесты с улучшениями, поддержкой новых платформ и расширением функциональности.

⭐ Если проект помогает вам работать с CSV — поставьте звёздочку!

