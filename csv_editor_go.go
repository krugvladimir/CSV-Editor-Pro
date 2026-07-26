// csv_editor_go.go — редактор CSV (табличный режим) на Go

package main

import (
	"bufio"
	"encoding/csv"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

// ANSI-цвета
const (
	green  = "\033[32m"
	red    = "\033[31m"
	yellow = "\033[33m"
	cyan   = "\033[36m"
	reset  = "\033[0m"
)

func printColor(text, color string) {
	fmt.Print(color + text + reset)
}

type CSVEditor struct {
	headers   []string
	data      [][]string
	filename  string
	delimiter rune
}

func NewCSVEditor() *CSVEditor {
	return &CSVEditor{delimiter: ','}
}

func (e *CSVEditor) detectDelimiter(line string) rune {
	if strings.Contains(line, "\t") {
		return '\t'
	}
	if strings.Contains(line, ";") {
		return ';'
	}
	return ','
}

func (e *CSVEditor) load(filename string) bool {
	file, err := os.Open(filename)
	if err != nil {
		printColor("Ошибка открытия файла: "+err.Error()+"\n", red)
		return false
	}
	defer file.Close()
	reader := csv.NewReader(file)
	reader.Comma = e.delimiter
	// Читаем первую строку для определения разделителя
	firstLine, err := reader.Read()
	if err != nil {
		printColor("Файл пуст или повреждён\n", red)
		return false
	}
	// Определяем разделитель по первой строке
	e.delimiter = e.detectDelimiter(strings.Join(firstLine, ""))
	reader.Comma = e.delimiter
	// Перезагружаем файл
	file.Seek(0, 0)
	reader = csv.NewReader(file)
	reader.Comma = e.delimiter
	all, err := reader.ReadAll()
	if err != nil {
		printColor("Ошибка чтения CSV: "+err.Error()+"\n", red)
		return false
	}
	if len(all) == 0 {
		printColor("Файл пуст\n", yellow)
		return false
	}
	e.headers = all[0]
	e.data = all[1:]
	e.filename = filename
	printColor(fmt.Sprintf("✅ Загружено %d строк, %d столбцов\n", len(e.data), len(e.headers)), green)
	return true
}

func (e *CSVEditor) save(filename string) bool {
	if filename == "" {
		filename = e.filename
	}
	if filename == "" {
		printColor("Нет файла для сохранения\n", red)
		return false
	}
	file, err := os.Create(filename)
	if err != nil {
		printColor("Ошибка создания файла: "+err.Error()+"\n", red)
		return false
	}
	defer file.Close()
	writer := csv.NewWriter(file)
	writer.Comma = e.delimiter
	all := [][]string{e.headers}
	all = append(all, e.data...)
	err = writer.WriteAll(all)
	if err != nil {
		printColor("Ошибка записи: "+err.Error()+"\n", red)
		return false
	}
	printColor("✅ Сохранено в "+filename+"\n", green)
	return true
}

func (e *CSVEditor) listData(limit int) {
	if len(e.headers) == 0 {
		printColor("Нет данных. Загрузите файл.\n", yellow)
		return
	}
	// Ширина столбцов
	colWidths := make([]int, len(e.headers))
	for i, h := range e.headers {
		colWidths[i] = len(h)
	}
	rowsToShow := len(e.data)
	if limit > 0 && limit < rowsToShow {
		rowsToShow = limit
	}
	for i := 0; i < rowsToShow; i++ {
		row := e.data[i]
		for j := 0; j < len(row) && j < len(colWidths); j++ {
			if len(row[j]) > colWidths[j] {
				colWidths[j] = len(row[j])
			}
		}
	}
	// Заголовок
	printColor("  #  ", cyan)
	for i, h := range e.headers {
		fmt.Printf("%-*s", colWidths[i]+2, h)
	}
	fmt.Println()
	printColor("-----", cyan)
	for _, w := range colWidths {
		fmt.Printf("%s", strings.Repeat("-", w+2))
	}
	fmt.Println()
	for i := 0; i < rowsToShow; i++ {
		row := e.data[i]
		fmt.Printf("%4d  ", i+1)
		for j := 0; j < len(e.headers); j++ {
			cell := ""
			if j < len(row) {
				cell = row[j]
			}
			fmt.Printf("%-*s", colWidths[j]+2, cell)
		}
		fmt.Println()
	}
}

func (e *CSVEditor) addRow(values []string) bool {
	if len(values) != len(e.headers) {
		printColor(fmt.Sprintf("Ожидается %d значений\n", len(e.headers)), red)
		return false
	}
	e.data = append(e.data, values)
	printColor("✅ Строка добавлена\n", green)
	return true
}

func (e *CSVEditor) deleteRow(rowNum int) bool {
	if rowNum < 1 || rowNum > len(e.data) {
		printColor("Неверный номер строки\n", red)
		return false
	}
	e.data = append(e.data[:rowNum-1], e.data[rowNum:]...)
	printColor(fmt.Sprintf("✅ Строка #%d удалена\n", rowNum), green)
	return true
}

func (e *CSVEditor) editCell(rowNum, colNum int, value string) bool {
	if rowNum < 1 || rowNum > len(e.data) {
		printColor("Неверный номер строки\n", red)
		return false
	}
	if colNum < 1 || colNum > len(e.headers) {
		printColor("Неверный номер столбца\n", red)
		return false
	}
	row := e.data[rowNum-1]
	for len(row) < colNum {
		row = append(row, "")
	}
	row[colNum-1] = value
	e.data[rowNum-1] = row
	printColor("✅ Ячейка обновлена\n", green)
	return true
}

func (e *CSVEditor) sortData(colNum int, reverse bool) bool {
	if colNum < 1 || colNum > len(e.headers) {
		printColor("Неверный номер столбца\n", red)
		return false
	}
	col := colNum - 1
	sort.Slice(e.data, func(i, j int) bool {
		va := ""
		vb := ""
		if col < len(e.data[i]) {
			va = e.data[i][col]
		}
		if col < len(e.data[j]) {
			vb = e.data[j][col]
		}
		if reverse {
			return va > vb
		}
		return va < vb
	})
	printColor("✅ Отсортировано\n", green)
	return true
}

func (e *CSVEditor) filterData(colNum int, value string) bool {
	if colNum < 1 || colNum > len(e.headers) {
		printColor("Неверный номер столбца\n", red)
		return false
	}
	col := colNum - 1
	var filtered [][]string
	for _, row := range e.data {
		if col < len(row) && row[col] == value {
			filtered = append(filtered, row)
		}
	}
	if len(filtered) == 0 {
		printColor("Нет строк, удовлетворяющих фильтру\n", yellow)
	} else {
		e.data = filtered
		printColor(fmt.Sprintf("✅ Отфильтровано, осталось %d строк\n", len(filtered)), green)
	}
	return true
}

func (e *CSVEditor) interactive() {
	printColor("📊 CSV Editor Pro — Go Edition\n", cyan)
	printColor("Команды: load <file>, list [n], add, delete <row>, edit <row> <col> <value>, sort <col> [desc], filter <col> <value>, save, saveas <file>, exit\n", reset)
	scanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print("> ")
		if !scanner.Scan() {
			break
		}
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		if len(parts) == 0 {
			continue
		}
		cmd := parts[0]
		switch cmd {
		case "exit":
			return
		case "load":
			if len(parts) < 2 {
				printColor("Укажите имя файла\n", red)
			} else {
				e.load(parts[1])
			}
		case "list":
			limit := -1
			if len(parts) > 1 {
				if l, err := strconv.Atoi(parts[1]); err == nil {
					limit = l
				}
			}
			e.listData(limit)
		case "add":
			if len(e.headers) == 0 {
				printColor("Нет данных. Загрузите файл.\n", yellow)
				continue
			}
			fmt.Printf("Введите %d значений через запятую: ", len(e.headers))
			if !scanner.Scan() {
				break
			}
			valsLine := scanner.Text()
			vals := strings.Split(valsLine, ",")
			e.addRow(vals)
		case "delete":
			if len(parts) < 2 {
				printColor("Укажите номер строки\n", red)
			} else {
				if row, err := strconv.Atoi(parts[1]); err == nil {
					e.deleteRow(row)
				} else {
					printColor("Неверный номер\n", red)
				}
			}
		case "edit":
			if len(parts) < 4 {
				printColor("Использование: edit <row> <col> <value>\n", red)
			} else {
				if row, err := strconv.Atoi(parts[1]); err == nil {
					if col, err := strconv.Atoi(parts[2]); err == nil {
						value := parts[3]
						e.editCell(row, col, value)
					}
				}
			}
		case "sort":
			if len(parts) < 2 {
				printColor("Укажите номер столбца\n", red)
			} else {
				if col, err := strconv.Atoi(parts[1]); err == nil {
					reverse := false
					if len(parts) > 2 && parts[2] == "desc" {
						reverse = true
					}
					e.sortData(col, reverse)
				}
			}
		case "filter":
			if len(parts) < 3 {
				printColor("Использование: filter <col> <value>\n", red)
			} else {
				if col, err := strconv.Atoi(parts[1]); err == nil {
					value := parts[2]
					e.filterData(col, value)
				}
			}
		case "save":
			e.save("")
		case "saveas":
			if len(parts) < 2 {
				printColor("Укажите имя файла\n", red)
			} else {
				e.save(parts[1])
			}
		default:
			printColor("Неизвестная команда\n", red)
		}
	}
}

func sort, slice etc. need import "sort". Add:
import "sort"

func main() {
	editor := NewCSVEditor()
	editor.interactive()
}
