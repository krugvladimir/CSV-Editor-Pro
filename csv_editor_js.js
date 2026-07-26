// csv_editor_js.js — редактор CSV (табличный режим) на JavaScript (Node.js)

const fs = require('fs');
const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    prompt: '> '
});

const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    cyan: '\x1b[36m',
    reset: '\x1b[0m'
};

function printColor(text, color) {
    console.log((colors[color] || '') + text + colors.reset);
}

class CSVEditor {
    constructor() {
        this.headers = [];
        this.data = [];
        this.filename = null;
        this.delimiter = ',';
    }

    detectDelimiter(line) {
        if (line.includes('\t')) return '\t';
        if (line.includes(';')) return ';';
        return ',';
    }

    splitLine(line) {
        return line.split(this.delimiter).map(s => s.trim());
    }

    load(filename) {
        try {
            const content = fs.readFileSync(filename, 'utf8');
            const lines = content.split('\n').filter(l => l.trim());
            if (lines.length === 0) {
                printColor('Файл пуст', 'red');
                return false;
            }
            this.delimiter = this.detectDelimiter(lines[0]);
            this.headers = this.splitLine(lines[0]);
            this.data = [];
            for (let i = 1; i < lines.length; i++) {
                if (lines[i].trim()) {
                    this.data.push(this.splitLine(lines[i]));
                }
            }
            this.filename = filename;
            printColor(`✅ Загружено ${this.data.length} строк, ${this.headers.length} столбцов`, 'green');
            return true;
        } catch (e) {
            printColor(`Ошибка загрузки: ${e.message}`, 'red');
            return false;
        }
    }

    save(filename) {
        const fname = filename || this.filename;
        if (!fname) {
            printColor('Нет файла для сохранения', 'red');
            return false;
        }
        try {
            let output = this.headers.join(this.delimiter) + '\n';
            for (const row of this.data) {
                output += row.join(this.delimiter) + '\n';
            }
            fs.writeFileSync(fname, output);
            printColor(`✅ Сохранено в ${fname}`, 'green');
            return true;
        } catch (e) {
            printColor(`Ошибка сохранения: ${e.message}`, 'red');
            return false;
        }
    }

    listData(limit) {
        if (this.headers.length === 0) {
            printColor('Нет данных. Загрузите файл.', 'yellow');
            return;
        }
        const colWidths = this.headers.map(h => h.length);
        const rowsToShow = limit ? Math.min(limit, this.data.length) : this.data.length;
        for (let i = 0; i < rowsToShow; i++) {
            const row = this.data[i];
            for (let j = 0; j < row.length && j < colWidths.length; j++) {
                colWidths[j] = Math.max(colWidths[j], row[j].length);
            }
        }
        // Заголовок
        printColor('  #  ' + this.headers.map((h, i) => h.padEnd(colWidths[i] + 2)).join(''), 'cyan');
        console.log('  ' + '-'.repeat(5 + colWidths.reduce((a, b) => a + b + 2, 0)));
        for (let i = 0; i < rowsToShow; i++) {
            const row = this.data[i];
            process.stdout.write(`${String(i+1).padStart(4)}  `);
            for (let j = 0; j < this.headers.length; j++) {
                const cell = (j < row.length) ? row[j] : '';
                process.stdout.write(cell.padEnd(colWidths[j] + 2));
            }
            console.log();
        }
    }

    addRow(values) {
        if (values.length !== this.headers.length) {
            printColor(`Ожидается ${this.headers.length} значений`, 'red');
            return false;
        }
        this.data.push(values);
        printColor('✅ Строка добавлена', 'green');
        return true;
    }

    deleteRow(rowNum) {
        if (rowNum < 1 || rowNum > this.data.length) {
            printColor('Неверный номер строки', 'red');
            return false;
        }
        this.data.splice(rowNum - 1, 1);
        printColor(`✅ Строка #${rowNum} удалена`, 'green');
        return true;
    }

    editCell(rowNum, colNum, value) {
        if (rowNum < 1 || rowNum > this.data.length) {
            printColor('Неверный номер строки', 'red');
            return false;
        }
        if (colNum < 1 || colNum > this.headers.length) {
            printColor('Неверный номер столбца', 'red');
            return false;
        }
        const row = this.data[rowNum - 1];
        while (row.length < colNum) row.push('');
        row[colNum - 1] = value;
        printColor('✅ Ячейка обновлена', 'green');
        return true;
    }

    sortData(colNum, reverse) {
        if (colNum < 1 || colNum > this.headers.length) {
            printColor('Неверный номер столбца', 'red');
            return false;
        }
        const col = colNum - 1;
        this.data.sort((a, b) => {
            const va = (col < a.length) ? a[col] : '';
            const vb = (col < b.length) ? b[col] : '';
            if (reverse) return vb.localeCompare(va);
            return va.localeCompare(vb);
        });
        printColor('✅ Отсортировано', 'green');
        return true;
    }

    filterData(colNum, value) {
        if (colNum < 1 || colNum > this.headers.length) {
            printColor('Неверный номер столбца', 'red');
            return false;
        }
        const col = colNum - 1;
        const filtered = this.data.filter(row => col < row.length && row[col] === value);
        if (filtered.length === 0) {
            printColor('Нет строк, удовлетворяющих фильтру', 'yellow');
        } else {
            this.data = filtered;
            printColor(`✅ Отфильтровано, осталось ${filtered.length} строк`, 'green');
        }
        return true;
    }

    interactive() {
        printColor('📊 CSV Editor Pro — JavaScript Edition', 'cyan');
        printColor('Команды: load <file>, list [n], add, delete <row>, edit <row> <col> <value>, sort <col> [desc], filter <col> <value>, save, saveas <file>, exit', 'reset');
        rl.prompt();

        rl.on('line', (line) => {
            const parts = line.trim().split(/\s+/);
            const cmd = parts[0];
            const arg = parts.slice(1).join(' ');
            const args = parts.slice(1);
            switch (cmd) {
                case 'exit':
                    rl.close();
                    return;
                case 'load':
                    if (args.length === 0) printColor('Укажите имя файла', 'red');
                    else this.load(args[0]);
                    break;
                case 'list':
                    const limit = args.length > 0 ? parseInt(args[0]) : undefined;
                    this.listData(limit);
                    break;
                case 'add':
                    if (this.headers.length === 0) {
                        printColor('Нет данных. Загрузите файл.', 'yellow');
                        break;
                    }
                    rl.question(`Введите ${this.headers.length} значений через запятую: `, (answer) => {
                        const vals = answer.split(',').map(s => s.trim());
                        this.addRow(vals);
                        rl.prompt();
                    });
                    return;
                case 'delete':
                    if (args.length === 0) printColor('Укажите номер строки', 'red');
                    else {
                        const row = parseInt(args[0]);
                        if (!isNaN(row)) this.deleteRow(row);
                        else printColor('Неверный номер', 'red');
                    }
                    break;
                case 'edit':
                    if (args.length < 3) printColor('Использование: edit <row> <col> <value>', 'red');
                    else {
                        const row = parseInt(args[0]);
                        const col = parseInt(args[1]);
                        const value = args[2];
                        if (!isNaN(row) && !isNaN(col)) this.editCell(row, col, value);
                        else printColor('Неверные аргументы', 'red');
                    }
                    break;
                case 'sort':
                    if (args.length === 0) printColor('Укажите номер столбца', 'red');
                    else {
                        const col = parseInt(args[0]);
                        if (!isNaN(col)) {
                            const reverse = args[1] && args[1].toLowerCase() === 'desc';
                            this.sortData(col, reverse);
                        } else printColor('Неверный номер', 'red');
                    }
                    break;
                case 'filter':
                    if (args.length < 2) printColor('Использование: filter <col> <value>', 'red');
                    else {
                        const col = parseInt(args[0]);
                        const value = args[1];
                        if (!isNaN(col)) this.filterData(col, value);
                        else printColor('Неверный номер', 'red');
                    }
                    break;
                case 'save':
                    this.save();
                    break;
                case 'saveas':
                    if (args.length === 0) printColor('Укажите имя файла', 'red');
                    else this.save(args[0]);
                    break;
                default:
                    printColor('Неизвестная команда', 'red');
            }
            rl.prompt();
        }).on('close', () => {
            console.log('До свидания!');
            process.exit(0);
        });
    }
}

const editor = new CSVEditor();
editor.interactive();
