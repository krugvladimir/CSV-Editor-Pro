// CSVEditorCSharp.cs — редактор CSV (табличный режим) на C#

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

class CSVEditorCSharp
{
    private List<string> headers = new List<string>();
    private List<List<string>> data = new List<List<string>>();
    private string filename;
    private char delimiter = ',';

    private void PrintColor(string text, string color)
    {
        ConsoleColor cc = ConsoleColor.White;
        switch (color)
        {
            case "green": cc = ConsoleColor.Green; break;
            case "red": cc = ConsoleColor.Red; break;
            case "yellow": cc = ConsoleColor.Yellow; break;
            case "blue": cc = ConsoleColor.Blue; break;
            case "cyan": cc = ConsoleColor.Cyan; break;
        }
        Console.ForegroundColor = cc;
        Console.WriteLine(text);
        Console.ResetColor();
    }

    private char DetectDelimiter(string line)
    {
        if (line.Contains('\t')) return '\t';
        if (line.Contains(';')) return ';';
        return ',';
    }

    private List<string> SplitLine(string line, char delim)
    {
        return line.Split(delim).ToList();
    }

    public bool Load(string filename)
    {
        try
        {
            using (var reader = new StreamReader(filename))
            {
                string line = reader.ReadLine();
                if (line == null)
                {
                    PrintColor("Файл пуст", "red");
                    return false;
                }
                delimiter = DetectDelimiter(line);
                headers = SplitLine(line, delimiter);
                data.Clear();
                while ((line = reader.ReadLine()) != null)
                {
                    if (!string.IsNullOrWhiteSpace(line))
                        data.Add(SplitLine(line, delimiter));
                }
                this.filename = filename;
                PrintColor($"✅ Загружено {data.Count} строк, {headers.Count} столбцов", "green");
                return true;
            }
        }
        catch (Exception e)
        {
            PrintColor($"Ошибка загрузки: {e.Message}", "red");
            return false;
        }
    }

    public bool Save(string filename = null)
    {
        if (filename == null) filename = this.filename;
        if (filename == null)
        {
            PrintColor("Нет файла для сохранения", "red");
            return false;
        }
        try
        {
            using (var writer = new StreamWriter(filename))
            {
                writer.WriteLine(string.Join(delimiter.ToString(), headers));
                foreach (var row in data)
                    writer.WriteLine(string.Join(delimiter.ToString(), row));
            }
            PrintColor($"✅ Сохранено в {filename}", "green");
            return true;
        }
        catch (Exception e)
        {
            PrintColor($"Ошибка сохранения: {e.Message}", "red");
            return false;
        }
    }

    public void ListData(int limit)
    {
        if (headers.Count == 0)
        {
            PrintColor("Нет данных. Загрузите файл.", "yellow");
            return;
        }
        int[] colWidths = headers.Select(h => h.Length).ToArray();
        int rowsToShow = limit > 0 ? Math.Min(limit, data.Count) : data.Count;
        for (int i = 0; i < rowsToShow; ++i)
        {
            var row = data[i];
            for (int j = 0; j < row.Count && j < colWidths.Length; ++j)
                colWidths[j] = Math.Max(colWidths[j], row[j].Length);
        }
        // Заголовок
        PrintColor("  #  " + string.Join("  ", headers.Select((h, i) => h.PadRight(colWidths[i] + 2))), "cyan");
        Console.WriteLine("  " + new string('-', 5 + colWidths.Sum() + 2 * colWidths.Length));
        for (int i = 0; i < rowsToShow; ++i)
        {
            var row = data[i];
            Console.Write($"{i+1,4}  ");
            for (int j = 0; j < headers.Count; ++j)
            {
                string cell = (j < row.Count) ? row[j] : "";
                Console.Write(cell.PadRight(colWidths[j] + 2));
            }
            Console.WriteLine();
        }
    }

    public bool AddRow(List<string> values)
    {
        if (values.Count != headers.Count)
        {
            PrintColor($"Ожидается {headers.Count} значений", "red");
            return false;
        }
        data.Add(values);
        PrintColor("✅ Строка добавлена", "green");
        return true;
    }

    public bool DeleteRow(int rowNum)
    {
        if (rowNum < 1 || rowNum > data.Count)
        {
            PrintColor("Неверный номер строки", "red");
            return false;
        }
        data.RemoveAt(rowNum - 1);
        PrintColor($"✅ Строка #{rowNum} удалена", "green");
        return true;
    }

    public bool EditCell(int rowNum, int colNum, string value)
    {
        if (rowNum < 1 || rowNum > data.Count)
        {
            PrintColor("Неверный номер строки", "red");
            return false;
        }
        if (colNum < 1 || colNum > headers.Count)
        {
            PrintColor("Неверный номер столбца", "red");
            return false;
        }
        var row = data[rowNum - 1];
        while (row.Count < colNum) row.Add("");
        row[colNum - 1] = value;
        PrintColor("✅ Ячейка обновлена", "green");
        return true;
    }

    public bool SortData(int colNum, bool reverse)
    {
        if (colNum < 1 || colNum > headers.Count)
        {
            PrintColor("Неверный номер столбца", "red");
            return false;
        }
        int col = colNum - 1;
        data.Sort((a, b) =>
        {
            string va = (col < a.Count) ? a[col] : "";
            string vb = (col < b.Count) ? b[col] : "";
            if (reverse) return string.Compare(vb, va, StringComparison.Ordinal);
            return string.Compare(va, vb, StringComparison.Ordinal);
        });
        PrintColor("✅ Отсортировано", "green");
        return true;
    }

    public bool FilterData(int colNum, string value)
    {
        if (colNum < 1 || colNum > headers.Count)
        {
            PrintColor("Неверный номер столбца", "red");
            return false;
        }
        int col = colNum - 1;
        var filtered = data.Where(row => col < row.Count && row[col] == value).ToList();
        if (filtered.Count == 0)
        {
            PrintColor("Нет строк, удовлетворяющих фильтру", "yellow");
        }
        else
        {
            data = filtered;
            PrintColor($"✅ Отфильтровано, осталось {filtered.Count} строк", "green");
        }
        return true;
    }

    public void Interactive()
    {
        PrintColor("📊 CSV Editor Pro — C# Edition", "cyan");
        PrintColor("Команды: load <file>, list [n], add, delete <row>, edit <row> <col> <value>, sort <col> [desc], filter <col> <value>, save, saveas <file>, exit", "white");
        while (true)
        {
            Console.Write("> ");
            string line = Console.ReadLine()?.Trim();
            if (string.IsNullOrEmpty(line)) continue;
            var parts = line.Split(' ');
            string cmd = parts[0].ToLower();
            try
            {
                switch (cmd)
                {
                    case "exit": return;
                    case "load":
                        if (parts.Length < 2) PrintColor("Укажите имя файла", "red");
                        else Load(parts[1]);
                        break;
                    case "list":
                        int limit = -1;
                        if (parts.Length > 1) limit = int.Parse(parts[1]);
                        ListData(limit);
                        break;
                    case "add":
                        if (headers.Count == 0)
                        {
                            PrintColor("Нет данных. Загрузите файл.", "yellow");
                            break;
                        }
                        Console.Write($"Введите {headers.Count} значений через запятую: ");
                        string valsLine = Console.ReadLine();
                        var vals = valsLine.Split(',').ToList();
                        AddRow(vals);
                        break;
                    case "delete":
                        if (parts.Length < 2) PrintColor("Укажите номер строки", "red");
                        else DeleteRow(int.Parse(parts[1]));
                        break;
                    case "edit":
                        if (parts.Length < 4) PrintColor("Использование: edit <row> <col> <value>", "red");
                        else EditCell(int.Parse(parts[1]), int.Parse(parts[2]), parts[3]);
                        break;
                    case "sort":
                        if (parts.Length < 2) PrintColor("Укажите номер столбца", "red");
                        else
                        {
                            bool reverse = parts.Length > 2 && parts[2].Equals("desc", StringComparison.OrdinalIgnoreCase);
                            SortData(int.Parse(parts[1]), reverse);
                        }
                        break;
                    case "filter":
                        if (parts.Length < 3) PrintColor("Использование: filter <col> <value>", "red");
                        else FilterData(int.Parse(parts[1]), parts[2]);
                        break;
                    case "save":
                        Save();
                        break;
                    case "saveas":
                        if (parts.Length < 2) PrintColor("Укажите имя файла", "red");
                        else Save(parts[1]);
                        break;
                    default:
                        PrintColor("Неизвестная команда", "red");
                        break;
                }
            }
            catch (Exception e)
            {
                PrintColor($"Ошибка: {e.Message}", "red");
            }
        }
    }

    public static void Main()
    {
        new CSVEditorCSharp().Interactive();
    }
}
