# csv_editor_python.py — редактор CSV (табличный режим) на Python

import csv
import os
import sys
import shutil
from collections import OrderedDict
from io import StringIO

try:
    from colorama import init, Fore, Style
    init(autoreset=True)
    HAS_COLOR = True
except ImportError:
    HAS_COLOR = False

class CSVEditor:
    def __init__(self):
        self.data = []          # список списков строк (без заголовка)
        self.headers = []
        self.filename = None
        self.delimiter = ','    # будет определено при загрузке
        self.quotechar = '"'

    def _print_color(self, text, color=None):
        if HAS_COLOR:
            colors = {
                'green': Fore.GREEN,
                'red': Fore.RED,
                'yellow': Fore.YELLOW,
                'blue': Fore.BLUE,
                'magenta': Fore.MAGENTA,
                'cyan': Fore.CYAN,
            }
            print(colors.get(color, '') + text + Style.RESET_ALL)
        else:
            print(text)

    def _detect_delimiter(self, first_line):
        # Проверяем наличие разделителей: запятая, точка с запятой, табуляция
        if '\t' in first_line:
            return '\t'
        elif ';' in first_line:
            return ';'
        else:
            return ','

    def load(self, filename):
        try:
            with open(filename, 'r', encoding='utf-8') as f:
                # Определяем разделитель по первой строке
                first_line = f.readline()
                f.seek(0)
                self.delimiter = self._detect_delimiter(first_line)
                reader = csv.reader(f, delimiter=self.delimiter, quotechar=self.quotechar)
                rows = list(reader)
                if rows:
                    self.headers = rows[0]
                    self.data = rows[1:]
                else:
                    self.headers = []
                    self.data = []
                self.filename = filename
                self._print_color(f"✅ Загружено {len(self.data)} строк, {len(self.headers)} столбцов", 'green')
                return True
        except Exception as e:
            self._print_color(f"Ошибка загрузки: {e}", 'red')
            return False

    def save(self, filename=None):
        if filename is None:
            filename = self.filename
        if not filename:
            self._print_color("Нет файла для сохранения. Укажите имя.", 'red')
            return False
        try:
            with open(filename, 'w', newline='', encoding='utf-8') as f:
                writer = csv.writer(f, delimiter=self.delimiter, quotechar=self.quotechar)
                if self.headers:
                    writer.writerow(self.headers)
                writer.writerows(self.data)
            self._print_color(f"✅ Сохранено в {filename}", 'green')
            return True
        except Exception as e:
            self._print_color(f"Ошибка сохранения: {e}", 'red')
            return False

    def list_data(self, limit=None):
        if not self.headers:
            self._print_color("Нет данных. Загрузите файл.", 'yellow')
            return
        # Определяем ширину столбцов
        col_widths = [len(h) for h in self.headers]
        for row in self.data[:limit] if limit else self.data:
            for i, cell in enumerate(row):
                if i < len(col_widths):
                    col_widths[i] = max(col_widths[i], len(str(cell)))
        # Заголовок
        header_line = "  #  " + "  ".join([f"{self.headers[i]:^{col_widths[i]}}" for i in range(len(self.headers))])
        self._print_color(header_line, 'cyan')
        sep_line = "-----" + "-" + "-".join(["-" * (col_widths[i] + 2) for i in range(len(self.headers))])
        print(sep_line)
        rows_to_show = self.data[:limit] if limit else self.data
        for idx, row in enumerate(rows_to_show, 1):
            # Дополняем строку до длины заголовка
            padded_row = row + [''] * (len(self.headers) - len(row))
            line = f"{idx:4}  " + "  ".join([f"{str(padded_row[i]):^{col_widths[i]}}" for i in range(len(self.headers))])
            print(line)

    def add_row(self, values):
        if len(values) != len(self.headers):
            self._print_color(f"Ожидается {len(self.headers)} значений, получено {len(values)}", 'red')
            return False
        self.data.append(values)
        self._print_color("✅ Строка добавлена", 'green')
        return True

    def delete_row(self, row_num):
        if row_num < 1 or row_num > len(self.data):
            self._print_color("Неверный номер строки", 'red')
            return False
        del self.data[row_num - 1]
        self._print_color(f"✅ Строка #{row_num} удалена", 'green')
        return True

    def edit_cell(self, row_num, col_num, value):
        if row_num < 1 or row_num > len(self.data):
            self._print_color("Неверный номер строки", 'red')
            return False
        if col_num < 1 or col_num > len(self.headers):
            self._print_color("Неверный номер столбца", 'red')
            return False
        self.data[row_num - 1][col_num - 1] = value
        self._print_color(f"✅ Ячейка ({row_num}, {col_num}) обновлена", 'green')
        return True

    def sort_data(self, col_num, reverse=False):
        if col_num < 1 or col_num > len(self.headers):
            self._print_color("Неверный номер столбца", 'red')
            return False
        self.data.sort(key=lambda row: row[col_num - 1] if col_num - 1 < len(row) else '', reverse=reverse)
        self._print_color(f"✅ Отсортировано по столбцу {col_num} {'(убывание)' if reverse else '(возрастание)'}", 'green')
        return True

    def filter_data(self, col_num, value):
        if col_num < 1 or col_num > len(self.headers):
            self._print_color("Неверный номер столбца", 'red')
            return False
        filtered = [row for row in self.data if col_num - 1 < len(row) and row[col_num - 1] == value]
        if not filtered:
            self._print_color("Нет строк, удовлетворяющих фильтру", 'yellow')
        else:
            # Показываем отфильтрованные строки
            self.data = filtered
            self._print_color(f"✅ Отфильтровано, осталось {len(filtered)} строк", 'green')
        return True

    def export_json(self, filename):
        import json
        try:
            with open(filename, 'w', encoding='utf-8') as f:
                json.dump(self.data, f, indent=2)
            self._print_color(f"✅ Экспортировано в JSON: {filename}", 'green')
        except Exception as e:
            self._print_color(f"Ошибка экспорта JSON: {e}", 'red')

    def interactive(self):
        self._print_color("📊 CSV Editor Pro — Python Edition", 'cyan')
        self._print_color("Команды: load <file>, list [n], add, delete <row>, edit <row> <col> <value>, sort <col> [desc], filter <col> <value>, save, saveas <file>, exportjson <file>, exit", 'white')
        while True:
            try:
                cmd = input("> ").strip()
                if not cmd:
                    continue
                parts = cmd.split()
                command = parts[0].lower()
                if command == "exit":
                    break
                elif command == "load":
                    if len(parts) < 2:
                        self._print_color("Укажите имя файла", 'red')
                    else:
                        self.load(parts[1])
                elif command == "list":
                    limit = None
                    if len(parts) > 1:
                        try:
                            limit = int(parts[1])
                        except:
                            self._print_color("Неверное число", 'red')
                            continue
                    self.list_data(limit)
                elif command == "add":
                    if not self.headers:
                        self._print_color("Нет данных. Загрузите файл.", 'yellow')
                        continue
                    print(f"Введите {len(self.headers)} значений через запятую:")
                    vals = input().strip().split(',')
                    if len(vals) != len(self.headers):
                        self._print_color(f"Ожидается {len(self.headers)} значений", 'red')
                    else:
                        self.add_row(vals)
                elif command == "delete":
                    if len(parts) < 2:
                        self._print_color("Укажите номер строки", 'red')
                    else:
                        try:
                            row = int(parts[1])
                            self.delete_row(row)
                        except:
                            self._print_color("Неверный номер", 'red')
                elif command == "edit":
                    if len(parts) < 4:
                        self._print_color("Использование: edit <row> <col> <value>", 'red')
                    else:
                        try:
                            row = int(parts[1])
                            col = int(parts[2])
                            value = parts[3]
                            self.edit_cell(row, col, value)
                        except:
                            self._print_color("Неверные аргументы", 'red')
                elif command == "sort":
                    if len(parts) < 2:
                        self._print_color("Укажите номер столбца", 'red')
                    else:
                        try:
                            col = int(parts[1])
                            reverse = False
                            if len(parts) > 2 and parts[2].lower() == "desc":
                                reverse = True
                            self.sort_data(col, reverse)
                        except:
                            self._print_color("Неверный аргумент", 'red')
                elif command == "filter":
                    if len(parts) < 3:
                        self._print_color("Использование: filter <col> <value>", 'red')
                    else:
                        try:
                            col = int(parts[1])
                            value = parts[2]
                            self.filter_data(col, value)
                        except:
                            self._print_color("Неверный аргумент", 'red')
                elif command == "save":
                    self.save()
                elif command == "saveas":
                    if len(parts) < 2:
                        self._print_color("Укажите имя файла", 'red')
                    else:
                        self.save(parts[1])
                elif command == "exportjson":
                    if len(parts) < 2:
                        self._print_color("Укажите имя файла", 'red')
                    else:
                        self.export_json(parts[1])
                else:
                    self._print_color("Неизвестная команда", 'red')
            except KeyboardInterrupt:
                print("\nВыход...")
                break

if __name__ == "__main__":
    editor = CSVEditor()
    editor.interactive()
