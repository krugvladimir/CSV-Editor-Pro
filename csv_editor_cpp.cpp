// csv_editor_cpp.cpp — редактор CSV (табличный режим) на C++

#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
#include <algorithm>
#include <iomanip>
#include <cctype>
#include <limits>

#ifdef _WIN32
#include <windows.h>
#else
#include <unistd.h>
#endif

using namespace std;

// Цветной вывод (ANSI)
void set_color(const string& color) {
#ifdef _WIN32
    // Для Windows можно использовать SetConsoleTextAttribute
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    int c = 7; // белый
    if (color == "green") c = 10;
    else if (color == "red") c = 12;
    else if (color == "yellow") c = 14;
    else if (color == "blue") c = 9;
    else if (color == "cyan") c = 11;
    SetConsoleTextAttribute(hConsole, c);
#else
    if (color == "green") cout << "\033[32m";
    else if (color == "red") cout << "\033[31m";
    else if (color == "yellow") cout << "\033[33m";
    else if (color == "blue") cout << "\033[34m";
    else if (color == "cyan") cout << "\033[36m";
#endif
}

void reset_color() {
#ifdef _WIN32
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, 7);
#else
    cout << "\033[0m";
#endif
}

void print_color(const string& text, const string& color) {
    set_color(color);
    cout << text;
    reset_color();
}

class CSVEditor {
private:
    vector<string> headers;
    vector<vector<string>> data;
    string filename;
    char delimiter = ',';

    char detect_delimiter(const string& first_line) {
        if (first_line.find('\t') != string::npos) return '\t';
        if (first_line.find(';') != string::npos) return ';';
        return ',';
    }

    vector<string> split(const string& line, char delim) {
        vector<string> tokens;
        stringstream ss(line);
        string token;
        while (getline(ss, token, delim)) {
            tokens.push_back(token);
        }
        return tokens;
    }

public:
    bool load(const string& filename) {
        ifstream file(filename);
        if (!file.is_open()) {
            print_color("Не удалось открыть файл\n", "red");
            return false;
        }
        string line;
        getline(file, line);
        if (line.empty()) {
            print_color("Файл пуст\n", "red");
            file.close();
            return false;
        }
        delimiter = detect_delimiter(line);
        headers = split(line, delimiter);
        data.clear();
        while (getline(file, line)) {
            if (!line.empty()) {
                data.push_back(split(line, delimiter));
            }
        }
        file.close();
        this->filename = filename;
        print_color("✅ Загружено " + to_string(data.size()) + " строк, " + to_string(headers.size()) + " столбцов\n", "green");
        return true;
    }

    bool save(const string& filename = "") {
        string out = filename.empty() ? this->filename : filename;
        if (out.empty()) {
            print_color("Нет файла для сохранения.\n", "red");
            return false;
        }
        ofstream file(out);
        if (!file.is_open()) {
            print_color("Не удалось создать файл\n", "red");
            return false;
        }
        for (size_t i = 0; i < headers.size(); ++i) {
            if (i) file << delimiter;
            file << headers[i];
        }
        file << "\n";
        for (const auto& row : data) {
            for (size_t i = 0; i < row.size(); ++i) {
                if (i) file << delimiter;
                file << row[i];
            }
            file << "\n";
        }
        file.close();
        print_color("✅ Сохранено в " + out + "\n", "green");
        return true;
    }

    void list_data(int limit = -1) {
        if (headers.empty()) {
            print_color("Нет данных. Загрузите файл.\n", "yellow");
            return;
        }
        // Ширина столбцов
        vector<size_t> col_widths;
        for (const auto& h : headers) col_widths.push_back(h.size());
        for (size_t i = 0; i < data.size() && (limit == -1 || i < (size_t)limit); ++i) {
            const auto& row = data[i];
            for (size_t j = 0; j < row.size() && j < col_widths.size(); ++j) {
                col_widths[j] = max(col_widths[j], row[j].size());
            }
        }
        // Заголовок
        print_color("  #  ", "cyan");
        for (size_t i = 0; i < headers.size(); ++i) {
            cout << setw(col_widths[i] + 2) << headers[i];
        }
        cout << "\n";
        print_color("-----", "cyan");
        for (size_t i = 0; i < headers.size(); ++i) {
            cout << setw(col_widths[i] + 2) << string(col_widths[i] + 2, '-');
        }
        cout << "\n";
        size_t count = 0;
        for (const auto& row : data) {
            if (limit != -1 && (size_t)limit <= count) break;
            ++count;
            cout << setw(4) << count << "  ";
            for (size_t i = 0; i < headers.size(); ++i) {
                string cell = (i < row.size()) ? row[i] : "";
                cout << setw(col_widths[i] + 2) << cell;
            }
            cout << "\n";
        }
    }

    bool add_row(const vector<string>& values) {
        if (values.size() != headers.size()) {
            print_color("Ожидается " + to_string(headers.size()) + " значений\n", "red");
            return false;
        }
        data.push_back(values);
        print_color("✅ Строка добавлена\n", "green");
        return true;
    }

    bool delete_row(int row_num) {
        if (row_num < 1 || row_num > (int)data.size()) {
            print_color("Неверный номер строки\n", "red");
            return false;
        }
        data.erase(data.begin() + row_num - 1);
        print_color("✅ Строка #" + to_string(row_num) + " удалена\n", "green");
        return true;
    }

    bool edit_cell(int row_num, int col_num, const string& value) {
        if (row_num < 1 || row_num > (int)data.size()) {
            print_color("Неверный номер строки\n", "red");
            return false;
        }
        if (col_num < 1 || col_num > (int)headers.size()) {
            print_color("Неверный номер столбца\n", "red");
            return false;
        }
        auto& row = data[row_num - 1];
        if (col_num - 1 >= (int)row.size()) row.resize(col_num);
        row[col_num - 1] = value;
        print_color("✅ Ячейка обновлена\n", "green");
        return true;
    }

    bool sort_data(int col_num, bool reverse = false) {
        if (col_num < 1 || col_num > (int)headers.size()) {
            print_color("Неверный номер столбца\n", "red");
            return false;
        }
        sort(data.begin(), data.end(), [col_num, reverse](const vector<string>& a, const vector<string>& b) {
            string val_a = (col_num-1 < (int)a.size()) ? a[col_num-1] : "";
            string val_b = (col_num-1 < (int)b.size()) ? b[col_num-1] : "";
            if (reverse) return val_a > val_b;
            return val_a < val_b;
        });
        print_color("✅ Отсортировано\n", "green");
        return true;
    }

    bool filter_data(int col_num, const string& value) {
        if (col_num < 1 || col_num > (int)headers.size()) {
            print_color("Неверный номер столбца\n", "red");
            return false;
        }
        vector<vector<string>> filtered;
        for (const auto& row : data) {
            if (col_num-1 < (int)row.size() && row[col_num-1] == value) {
                filtered.push_back(row);
            }
        }
        if (filtered.empty()) {
            print_color("Нет строк, удовлетворяющих фильтру\n", "yellow");
        } else {
            data = filtered;
            print_color("✅ Отфильтровано, осталось " + to_string(filtered.size()) + " строк\n", "green");
        }
        return true;
    }

    void interactive() {
        print_color("📊 CSV Editor Pro — C++ Edition\n", "cyan");
        print_color("Команды: load <file>, list [n], add, delete <row>, edit <row> <col> <value>, sort <col> [desc], filter <col> <value>, save, saveas <file>, exit\n", "white");
        string line;
        while (true) {
            cout << "> ";
            getline(cin, line);
            if (line.empty()) continue;
            stringstream ss(line);
            string cmd;
            ss >> cmd;
            if (cmd == "exit") break;
            else if (cmd == "load") {
                string fname;
                ss >> fname;
                if (fname.empty()) print_color("Укажите имя файла\n", "red");
                else load(fname);
            } else if (cmd == "list") {
                int limit = -1;
                if (ss >> limit) { }
                list_data(limit);
            } else if (cmd == "add") {
                if (headers.empty()) {
                    print_color("Нет данных. Загрузите файл.\n", "yellow");
                    continue;
                }
                print_color("Введите " + to_string(headers.size()) + " значений через запятую: ", "white");
                string line_vals;
                getline(cin, line_vals);
                vector<string> vals;
                stringstream vals_ss(line_vals);
                string token;
                while (getline(vals_ss, token, ',')) {
                    vals.push_back(token);
                }
                add_row(vals);
            } else if (cmd == "delete") {
                int row;
                if (ss >> row) delete_row(row);
                else print_color("Укажите номер строки\n", "red");
            } else if (cmd == "edit") {
                int row, col;
                string value;
                if (ss >> row >> col >> value) edit_cell(row, col, value);
                else print_color("Использование: edit <row> <col> <value>\n", "red");
            } else if (cmd == "sort") {
                int col;
                if (ss >> col) {
                    string desc;
                    bool reverse = false;
                    if (ss >> desc && desc == "desc") reverse = true;
                    sort_data(col, reverse);
                } else print_color("Укажите номер столбца\n", "red");
            } else if (cmd == "filter") {
                int col;
                string value;
                if (ss >> col >> value) filter_data(col, value);
                else print_color("Использование: filter <col> <value>\n", "red");
            } else if (cmd == "save") {
                save();
            } else if (cmd == "saveas") {
                string fname;
                if (ss >> fname) save(fname);
                else print_color("Укажите имя файла\n", "red");
            } else {
                print_color("Неизвестная команда\n", "red");
            }
        }
    }
};

int main() {
    CSVEditor editor;
    editor.interactive();
    return 0;
}
