// CSVEditorJava.java — редактор CSV (табличный режим) на Java

import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.stream.*;

public class CSVEditorJava {
    private List<String> headers = new ArrayList<>();
    private List<List<String>> data = new ArrayList<>();
    private String filename = null;
    private char delimiter = ',';

    private void printColor(String text, String color) {
        String code = "";
        switch (color) {
            case "green": code = "\u001B[32m"; break;
            case "red": code = "\u001B[31m"; break;
            case "yellow": code = "\u001B[33m"; break;
            case "blue": code = "\u001B[34m"; break;
            case "cyan": code = "\u001B[36m"; break;
            default: code = "";
        }
        System.out.println(code + text + "\u001B[0m");
    }

    private char detectDelimiter(String line) {
        if (line.indexOf('\t') != -1) return '\t';
        if (line.indexOf(';') != -1) return ';';
        return ',';
    }

    private List<String> splitLine(String line, char delim) {
        return Arrays.asList(line.split(String.valueOf(delim)));
    }

    public boolean load(String filename) {
        try (BufferedReader reader = new BufferedReader(new FileReader(filename))) {
            String line = reader.readLine();
            if (line == null) {
                printColor("Файл пуст", "red");
                return false;
            }
            delimiter = detectDelimiter(line);
            headers = splitLine(line, delimiter);
            data.clear();
            while ((line = reader.readLine()) != null) {
                if (!line.trim().isEmpty()) {
                    data.add(splitLine(line, delimiter));
                }
            }
            this.filename = filename;
            printColor("✅ Загружено " + data.size() + " строк, " + headers.size() + " столбцов", "green");
            return true;
        } catch (IOException e) {
            printColor("Ошибка загрузки: " + e.getMessage(), "red");
            return false;
        }
    }

    public boolean save(String filename) {
        if (filename == null) filename = this.filename;
        if (filename == null) {
            printColor("Нет файла для сохранения", "red");
            return false;
        }
        try (PrintWriter writer = new PrintWriter(filename)) {
            writer.println(String.join(String.valueOf(delimiter), headers));
            for (List<String> row : data) {
                writer.println(String.join(String.valueOf(delimiter), row));
            }
            printColor("✅ Сохранено в " + filename, "green");
            return true;
        } catch (IOException e) {
            printColor("Ошибка сохранения: " + e.getMessage(), "red");
            return false;
        }
    }

    public void listData(int limit) {
        if (headers.isEmpty()) {
            printColor("Нет данных. Загрузите файл.", "yellow");
            return;
        }
        // Ширина столбцов
        int[] colWidths = headers.stream().mapToInt(String::length).toArray();
        int rowsToShow = limit > 0 ? Math.min(limit, data.size()) : data.size();
        for (int i = 0; i < rowsToShow; ++i) {
            List<String> row = data.get(i);
            for (int j = 0; j < row.size() && j < colWidths.length; ++j) {
                colWidths[j] = Math.max(colWidths[j], row.get(j).length());
            }
        }
        // Заголовок
        printColor("  #  " + IntStream.range(0, headers.size())
                .mapToObj(i -> String.format("%-" + (colWidths[i]+2) + "s", headers.get(i)))
                .collect(Collectors.joining()), "cyan");
        System.out.println("  " + "-".repeat(5 + IntStream.of(colWidths).sum() + 2*colWidths.length));
        for (int i = 0; i < rowsToShow; ++i) {
            List<String> row = data.get(i);
            System.out.printf("%4d  ", i+1);
            for (int j = 0; j < headers.size(); ++j) {
                String cell = (j < row.size()) ? row.get(j) : "";
                System.out.printf("%-" + (colWidths[j]+2) + "s", cell);
            }
            System.out.println();
        }
    }

    public boolean addRow(List<String> values) {
        if (values.size() != headers.size()) {
            printColor("Ожидается " + headers.size() + " значений", "red");
            return false;
        }
        data.add(values);
        printColor("✅ Строка добавлена", "green");
        return true;
    }

    public boolean deleteRow(int rowNum) {
        if (rowNum < 1 || rowNum > data.size()) {
            printColor("Неверный номер строки", "red");
            return false;
        }
        data.remove(rowNum - 1);
        printColor("✅ Строка #" + rowNum + " удалена", "green");
        return true;
    }

    public boolean editCell(int rowNum, int colNum, String value) {
        if (rowNum < 1 || rowNum > data.size()) {
            printColor("Неверный номер строки", "red");
            return false;
        }
        if (colNum < 1 || colNum > headers.size()) {
            printColor("Неверный номер столбца", "red");
            return false;
        }
        List<String> row = data.get(rowNum - 1);
        while (row.size() < colNum) row.add("");
        row.set(colNum - 1, value);
        printColor("✅ Ячейка обновлена", "green");
        return true;
    }

    public boolean sortData(int colNum, boolean reverse) {
        if (colNum < 1 || colNum > headers.size()) {
            printColor("Неверный номер столбца", "red");
            return false;
        }
        int col = colNum - 1;
        data.sort((a, b) -> {
            String va = (col < a.size()) ? a.get(col) : "";
            String vb = (col < b.size()) ? b.get(col) : "";
            if (reverse) return vb.compareTo(va);
            return va.compareTo(vb);
        });
        printColor("✅ Отсортировано", "green");
        return true;
    }

    public boolean filterData(int colNum, String value) {
        if (colNum < 1 || colNum > headers.size()) {
            printColor("Неверный номер столбца", "red");
            return false;
        }
        int col = colNum - 1;
        List<List<String>> filtered = data.stream()
                .filter(row -> col < row.size() && row.get(col).equals(value))
                .collect(Collectors.toList());
        if (filtered.isEmpty()) {
            printColor("Нет строк, удовлетворяющих фильтру", "yellow");
        } else {
            data = filtered;
            printColor("✅ Отфильтровано, осталось " + filtered.size() + " строк", "green");
        }
        return true;
    }

    public void interactive() {
        printColor("📊 CSV Editor Pro — Java Edition", "cyan");
        printColor("Команды: load <file>, list [n], add, delete <row>, edit <row> <col> <value>, sort <col> [desc], filter <col> <value>, save, saveas <file>, exit", "white");
        Scanner sc = new Scanner(System.in);
        while (true) {
            System.out.print("> ");
            String line = sc.nextLine().trim();
            if (line.isEmpty()) continue;
            String[] parts = line.split("\\s+");
            String cmd = parts[0].toLowerCase();
            try {
                switch (cmd) {
                    case "exit": return;
                    case "load":
                        if (parts.length < 2) printColor("Укажите имя файла", "red");
                        else load(parts[1]);
                        break;
                    case "list": {
                        int limit = -1;
                        if (parts.length > 1) limit = Integer.parseInt(parts[1]);
                        listData(limit);
                        break;
                    }
                    case "add":
                        if (headers.isEmpty()) {
                            printColor("Нет данных. Загрузите файл.", "yellow");
                            break;
                        }
                        System.out.print("Введите " + headers.size() + " значений через запятую: ");
                        String valsLine = sc.nextLine();
                        List<String> vals = Arrays.asList(valsLine.split(","));
                        addRow(vals);
                        break;
                    case "delete":
                        if (parts.length < 2) printColor("Укажите номер строки", "red");
                        else deleteRow(Integer.parseInt(parts[1]));
                        break;
                    case "edit":
                        if (parts.length < 4) printColor("Использование: edit <row> <col> <value>", "red");
                        else editCell(Integer.parseInt(parts[1]), Integer.parseInt(parts[2]), parts[3]);
                        break;
                    case "sort":
                        if (parts.length < 2) printColor("Укажите номер столбца", "red");
                        else {
                            boolean reverse = parts.length > 2 && parts[2].equalsIgnoreCase("desc");
                            sortData(Integer.parseInt(parts[1]), reverse);
                        }
                        break;
                    case "filter":
                        if (parts.length < 3) printColor("Использование: filter <col> <value>", "red");
                        else filterData(Integer.parseInt(parts[1]), parts[2]);
                        break;
                    case "save":
                        save(null);
                        break;
                    case "saveas":
                        if (parts.length < 2) printColor("Укажите имя файла", "red");
                        else save(parts[1]);
                        break;
                    default:
                        printColor("Неизвестная команда", "red");
                }
            } catch (NumberFormatException e) {
                printColor("Неверный аргумент", "red");
            }
        }
        sc.close();
    }

    public static void main(String[] args) {
        new CSVEditorJava().interactive();
    }
}
