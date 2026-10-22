use std::path::Path;

use anyhow::Result;

pub fn generate_super_template(ext: &str) -> &'static str {
    match ext {
        "c" => {
            "\
#include <stdio.h>
#include <string.h>

// Struct (OOP-like in C)
typedef struct {
    char name[64];
    int age;
    float score;
} Student;

// Function to print student info
void print_student(Student s) {
    printf(\"%s (%d): %.1f\\n\", s.name, s.age, s.score);
}

int main(void) {
    // Variables
    int count = 5;
    char message[] = \"Hello, C!\";

    // Array of structs
    Student students[] = {
        {\"Alice\", 20, 92.5},
        {\"Bob\",   22, 85.0},
        {\"Charlie\", 21, 78.3}
    };
    int n = sizeof(students) / sizeof(students[0]);

    // For loop over array
    printf(\"%s\\n\", message);
    printf(\"Students:\\n\");
    for (int i = 0; i < n; i++) {
        print_student(students[i]);
    }

    // Working with numeric array
    int scores[] = {88, 95, 70, 82, 91};
    int sum = 0;
    for (int i = 0; i < 5; i++) {
        sum += scores[i];
    }
    printf(\"Average score: %.1f\\n\", (float)sum / 5);

    return 0;
}
"
        }
        "cpp" | "cc" | "cxx" => {
            "\
#include <iostream>
#include <vector>
#include <string>

// Class (OOP)
class Student {
public:
    Student(const std::string& name, int age, double score)
        : name_(name), age_(age), score_(score) {}

    void print() const {
        std::cout << name_ << \" (\" << age_ << \"): \" << score_ << std::endl;
    }

private:
    std::string name_;
    int age_;
    double score_;
};

int main() {
    // Variables
    int count = 5;
    std::string message = \"Hello, C++!\";

    // Array / Vector
    std::vector<Student> students = {
        {\"Alice\", 20, 92.5},
        {\"Bob\",   22, 85.0},
        {\"Charlie\", 21, 78.3}
    };

    // For loop over array
    std::cout << message << std::endl;
    std::cout << \"Students:\" << std::endl;
    for (const auto& s : students) {
        s.print();
    }

    // Working with numeric array
    int scores[] = {88, 95, 70, 82, 91};
    int sum = 0;
    for (int score : scores) {
        sum += score;
    }
    std::cout << \"Average score: \" << (double)sum / 5 << std::endl;

    return 0;
}
"
        }
        "cs" | "csx" => {
            "\
using System;
using System.Collections.Generic;

// Class (OOP)
class Student {
    public string Name { get; set; }
    public int Age { get; set; }
    public double Score { get; set; }

    public Student(string name, int age, double score) {
        Name = name;
        Age = age;
        Score = score;
    }

    public void Print() {
        Console.WriteLine($\"{Name} ({Age}): {Score}\");
    }
}

class Program {
    static void Main() {
        // Variables
        int count = 5;
        string message = \"Hello, C#!\";

        // List of objects
        var students = new List<Student> {
            new Student(\"Alice\", 20, 92.5),
            new Student(\"Bob\",   22, 85.0),
            new Student(\"Charlie\", 21, 78.3)
        };

        // For each loop
        Console.WriteLine(message);
        Console.WriteLine(\"Students:\");
        foreach (var s in students) {
            s.Print();
        }

        // Working with numeric array
        int[] scores = {88, 95, 70, 82, 91};
        int sum = 0;
        for (int i = 0; i < scores.Length; i++) {
            sum += scores[i];
        }
        Console.WriteLine($\"Average score: {(double)sum / scores.Length:F1}\");
    }
}
"
        }
        "go" => {
            "\
package main

import \"fmt\"

// Struct (OOP-like in Go)
type Student struct {
    Name  string
    Age   int
    Score float64
}

func (s Student) Print() {
    fmt.Printf(\"%s (%d): %.1f\\n\", s.Name, s.Age, s.Score)
}

func main() {
    // Variables
    count := 5
    message := \"Hello, Go!\"

    // Slice of structs
    students := []Student{
        {\"Alice\", 20, 92.5},
        {\"Bob\",   22, 85.0},
        {\"Charlie\", 21, 78.3},
    }

    // For loop over slice
    fmt.Println(message)
    fmt.Println(\"Students:\")
    for _, s := range students {
        s.Print()
    }

    // Working with numeric slice
    scores := []int{88, 95, 70, 82, 91}
    sum := 0
    for _, v := range scores {
        sum += v
    }
    fmt.Printf(\"Average score: %.1f\\n\", float64(sum)/float64(len(scores)))
}
"
        }
        "java" => {
            "\
import java.util.*;

// Class (OOP)
class Student {
    private String name;
    private int age;
    private double score;

    public Student(String name, int age, double score) {
        this.name = name;
        this.age = age;
        this.score = score;
    }

    public void print() {
        System.out.printf(\"%s (%d): %.1f%n\", name, age, score);
    }
}

public class Main {
    public static void main(String[] args) {
        // Variables
        int count = 5;
        String message = \"Hello, Java!\";

        // ArrayList of objects
        List<Student> students = new ArrayList<>();
        students.add(new Student(\"Alice\", 20, 92.5));
        students.add(new Student(\"Bob\",   22, 85.0));
        students.add(new Student(\"Charlie\", 21, 78.3));

        // For each loop
        System.out.println(message);
        System.out.println(\"Students:\");
        for (Student s : students) {
            s.print();
        }

        // Working with numeric array
        int[] scores = {88, 95, 70, 82, 91};
        int sum = 0;
        for (int i = 0; i < scores.length; i++) {
            sum += scores[i];
        }
        System.out.printf(\"Average score: %.1f%n\", (double)sum / scores.length);
    }
}
"
        }
        "kt" | "kts" => {
            "\
// Data class (OOP in Kotlin)
data class Student(val name: String, val age: Int, val score: Double)

fun Student.print() = println(\"$name ($age): $score\")

fun main() {
    // Variables
    val count = 5
    val message = \"Hello, Kotlin!\"

    // List of objects
    val students = listOf(
        Student(\"Alice\", 20, 92.5),
        Student(\"Bob\",   22, 85.0),
        Student(\"Charlie\", 21, 78.3)
    )

    // For loop over list
    println(message)
    println(\"Students:\")
    for (s in students) {
        s.print()
    }

    // Working with numeric array
    val scores = intArrayOf(88, 95, 70, 82, 91)
    var sum = 0
    for (v in scores) {
        sum += v
    }
    println(\"Average score: ${sum.toDouble() / scores.size}\")
}
"
        }
        "swift" => {
            "\
import Foundation

// Struct (OOP)
struct Student {
    let name: String
    let age: Int
    let score: Double

    func printInfo() {
        print(\"\\(name) (\\(age)): \\(score)\")
    }
}

func main() {
    // Variables
    let count = 5
    let message = \"Hello, Swift!\"

    // Array of structs
    let students = [
        Student(name: \"Alice\", age: 20, score: 92.5),
        Student(name: \"Bob\",   age: 22, score: 85.0),
        Student(name: \"Charlie\", age: 21, score: 78.3)
    ]

    // For loop over array
    print(message)
    print(\"Students:\")
    for s in students {
        s.printInfo()
    }

    // Working with numeric array
    let scores = [88, 95, 70, 82, 91]
    var sum = 0
    for v in scores {
        sum += v
    }
    print(\"Average score: \\(Double(sum) / Double(scores.count))\")
}

main()
"
        }
        "dart" => {
            "\
// Class (OOP)
class Student {
  final String name;
  final int age;
  final double score;

  Student(this.name, this.age, this.score);

  void printInfo() {
    print('$name ($age): $score');
  }
}

void main() {
  // Variables
  var count = 5;
  var message = 'Hello, Dart!';

  // List of objects
  var students = [
    Student('Alice', 20, 92.5),
    Student('Bob',   22, 85.0),
    Student('Charlie', 21, 78.3),
  ];

  // For loop over list
  print(message);
  print('Students:');
  for (var s in students) {
    s.printInfo();
  }

  // Working with numeric list
  var scores = [88, 95, 70, 82, 91];
  var sum = 0;
  for (var v in scores) {
    sum += v;
  }
  print('Average score: ${sum / scores.length}');
}
"
        }
        "rs" => {
            "\
// Struct (OOP-like in Rust)
#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    score: f64,
}

impl Student {
    fn print(&self) {
        println!(\"{} ({}): {}\", self.name, self.age, self.score);
    }
}

fn main() {
    // Variables
    let count = 5;
    let message = \"Hello, Rust!\";

    // Vector of structs
    let students = vec![
        Student { name: \"Alice\".into(), age: 20, score: 92.5 },
        Student { name: \"Bob\".into(),   age: 22, score: 85.0 },
        Student { name: \"Charlie\".into(), age: 21, score: 78.3 },
    ];

    // For loop over vector
    println!(\"{}\", message);
    println!(\"Students:\");
    for s in &students {
        s.print();
    }

    // Working with numeric array
    let scores = [88, 95, 70, 82, 91];
    let sum: i32 = scores.iter().sum();
    println!(\"Average score: {}\", sum as f64 / scores.len() as f64);
}
"
        }
        "py" => {
            "\
# Class (OOP)
class Student:
    def __init__(self, name: str, age: int, score: float):
        self.name = name
        self.age = age
        self.score = score

    def print_info(self):
        print(f\"{self.name} ({self.age}): {self.score}\")


def main():
    # Variables
    count = 5
    message = \"Hello, Python!\"

    # List of objects
    students = [
        Student(\"Alice\", 20, 92.5),
        Student(\"Bob\",   22, 85.0),
        Student(\"Charlie\", 21, 78.3),
    ]

    # For loop over list
    print(message)
    print(\"Students:\")
    for s in students:
        s.print_info()

    # Working with numeric list
    scores = [88, 95, 70, 82, 91]
    total = 0
    for v in scores:
        total += v
    print(f\"Average score: {total / len(scores):.1f}\")


if __name__ == \"__main__\":
    main()
"
        }
        "js" => {
            "\
// Class (OOP in JavaScript)
class Student {
    constructor(name, age, score) {
        this.name = name;
        this.age = age;
        this.score = score;
    }

    print() {
        console.log(`${this.name} (${this.age}): ${this.score}`);
    }
}

function main() {
    // Variables
    const count = 5;
    const message = \"Hello, JavaScript!\";

    // Array of objects
    const students = [
        new Student(\"Alice\", 20, 92.5),
        new Student(\"Bob\",   22, 85.0),
        new Student(\"Charlie\", 21, 78.3),
    ];

    // For loop over array
    console.log(message);
    console.log(\"Students:\");
    for (const s of students) {
        s.print();
    }

    // Working with numeric array
    const scores = [88, 95, 70, 82, 91];
    let sum = 0;
    for (const v of scores) {
        sum += v;
    }
    console.log(`Average score: ${(sum / scores.length).toFixed(1)}`);
}

main();
"
        }
        "ts" => {
            "\
// Class (OOP)
class Student {
    constructor(
        public name: string,
        public age: number,
        public score: number
    ) {}

    print(): void {
        console.log(`${this.name} (${this.age}): ${this.score}`);
    }
}

function main(): void {
    // Variables
    const count: number = 5;
    const message: string = \"Hello, TypeScript!\";

    // Array of objects
    const students: Student[] = [
        new Student(\"Alice\", 20, 92.5),
        new Student(\"Bob\",   22, 85.0),
        new Student(\"Charlie\", 21, 78.3),
    ];

    // For loop over array
    console.log(message);
    console.log(\"Students:\");
    for (const s of students) {
        s.print();
    }

    // Working with numeric array
    const scores: number[] = [88, 95, 70, 82, 91];
    let sum: number = 0;
    for (const v of scores) {
        sum += v;
    }
    console.log(`Average score: ${(sum / scores.length).toFixed(1)}`);
}

main();
"
        }
        "rb" => {
            "\
# Class (OOP)
class Student
    attr_reader :name, :age, :score

    def initialize(name, age, score)
        @name = name
        @age = age
        @score = score
    end

    def print_info
        puts \"#{@name} (#{@age}): #{@score}\"
    end
end

def main
    # Variables
    count = 5
    message = \"Hello, Ruby!\"

    # Array of objects
    students = [
        Student.new(\"Alice\", 20, 92.5),
        Student.new(\"Bob\",   22, 85.0),
        Student.new(\"Charlie\", 21, 78.3),
    ]

    # For loop over array
    puts message
    puts \"Students:\"
    students.each { |s| s.print_info }

    # Working with numeric array
    scores = [88, 95, 70, 82, 91]
    total = 0
    scores.each { |v| total += v }
    puts \"Average score: #{total.to_f / scores.length}\"
end

main
"
        }
        "lua" => {
            "\
-- Table as object (OOP-like in Lua)
Student = {}
Student.__index = Student

function Student:new(name, age, score)
    return setmetatable({name = name, age = age, score = score}, self)
end

function Student:print()
    print(string.format(\"%s (%d): %.1f\", self.name, self.age, self.score))
end

function main()
    -- Variables
    local count = 5
    local message = \"Hello, Lua!\"

    -- Array (table) of objects
    local students = {
        Student:new(\"Alice\", 20, 92.5),
        Student:new(\"Bob\",   22, 85.0),
        Student:new(\"Charlie\", 21, 78.3),
    }

    -- For loop over array
    print(message)
    print(\"Students:\")
    for _, s in ipairs(students) do
        s:print()
    end

    -- Working with numeric array
    local scores = {88, 95, 70, 82, 91}
    local sum = 0
    for _, v in ipairs(scores) do
        sum = sum + v
    end
    print(string.format(\"Average score: %.1f\", sum / #scores))
end

main()
"
        }
        "php" => {
            "\
<?php

// Class (OOP)
class Student {
    public function __construct(
        public string $name,
        public int $age,
        public float $score
    ) {}

    public function print(): void {
        printf(\"%s (%d): %.1f\\n\", $this->name, $this->age, $this->score);
    }
}

function main(): void {
    // Variables
    $count = 5;
    $message = \"Hello, PHP!\";

    // Array of objects
    $students = [
        new Student(\"Alice\", 20, 92.5),
        new Student(\"Bob\",   22, 85.0),
        new Student(\"Charlie\", 21, 78.3),
    ];

    // For loop over array
    echo $message . \"\\n\";
    echo \"Students:\\n\";
    foreach ($students as $s) {
        $s->print();
    }

    // Working with numeric array
    $scores = [88, 95, 70, 82, 91];
    $sum = 0;
    foreach ($scores as $v) {
        $sum += $v;
    }
    printf(\"Average score: %.1f\\n\", $sum / count($scores));
}

main();
"
        }
        "zig" => {
            "\
const std = @import(\"std\");

// Struct (OOP-like)
const Student = struct {
    name: []const u8,
    age: u32,
    score: f64,

    fn print(self: Student, writer: anytype) !void {
        try writer.print(\"{s} ({d}): {d:.1}\\n\", .{self.name, self.age, self.score});
    }
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    // Variables
    const count = 5;
    const message = \"Hello, Zig!\";

    // Array of structs
    const students = [_]Student{
        Student{.name = \"Alice\", .age = 20, .score = 92.5},
        Student{.name = \"Bob\",   .age = 22, .score = 85.0},
        Student{.name = \"Charlie\", .age = 21, .score = 78.3},
    };

    const stdout = std.io.getStdOut().writer();

    // For loop over array
    try stdout.print(\"{s}\\n\", .{message});
    try stdout.print(\"Students:\\n\", .{});
    for (students) |s| {
        try s.print(stdout);
    }

    // Working with numeric array
    const scores = [_]i32{88, 95, 70, 82, 91};
    var sum: i32 = 0;
    for (scores) |v| {
        sum += v;
    }
    const avg = @as(f64, sum) / @as(f64, scores.len);
    try stdout.print(\"Average score: {d:.1}\\n\", .{avg});
}
"
        }
        "scala" => {
            "\
// Case class (OOP in Scala)
case class Student(name: String, age: Int, score: Double) {
    def printInfo(): Unit = println(s\"$name ($age): $score\")
}

object Main {
    def main(args: Array[String]): Unit = {
        // Variables
        val count = 5
        val message = \"Hello, Scala!\"

        // List of objects
        val students = List(
            Student(\"Alice\", 20, 92.5),
            Student(\"Bob\",   22, 85.0),
            Student(\"Charlie\", 21, 78.3)
        )

        // For loop over list
        println(message)
        println(\"Students:\")
        students.foreach(_.printInfo())

        // Working with numeric array
        val scores = Array(88, 95, 70, 82, 91)
        val sum = scores.sum
        println(f\"Average score: ${sum.toDouble / scores.length}%.1f\")
    }
}
"
        }
        "ex" | "exs" => {
            "\
defmodule Student do
  defstruct [:name, :age, :score]

      def print(%__MODULE__{name: name, age: age, score: score}) do
    IO.puts(\"#{name} (#{age}): #{score}\")
  end
end

defmodule Main do
  def main do
    # Variables
    count = 5
    message = \"Hello, Elixir!\"

    # List of structs
    students = [
      %Student{name: \"Alice\", age: 20, score: 92.5},
      %Student{name: \"Bob\",   age: 22, score: 85.0},
      %Student{name: \"Charlie\", age: 21, score: 78.3},
    ]

    # For loop over list
    IO.puts(message)
    IO.puts(\"Students:\")
    Enum.each(students, &Student.print/1)

    # Working with numeric list
    scores = [88, 95, 70, 82, 91]
    sum = Enum.sum(scores)
    IO.puts(\"Average score: #{sum / length(scores)}\")
  end
end

Main.main()
"
        }
        "nim" => {
            "\
type
  Student = object
    name: string
    age: int
    score: float

proc print(s: Student) =
  echo s.name, \" (\", s.age, \"): \", s.score

proc main() =
  # Variables
  let count = 5
  let message = \"Hello, Nim!\"

  # Sequence of objects
  var students = @[
    Student(name: \"Alice\", age: 20, score: 92.5),
    Student(name: \"Bob\",   age: 22, score: 85.0),
    Student(name: \"Charlie\", age: 21, score: 78.3),
  ]

  # For loop over sequence
  echo message
  echo \"Students:\"
  for s in students:
    s.print()

  # Working with numeric sequence
  var scores = @[88, 95, 70, 82, 91]
  var sum = 0
  for v in scores:
    sum += v
  echo \"Average score: \", sum / scores.len

when isMainModule:
  main()
"
        }
        "clj" | "cljs" => {
            "\
;; Record (OOP-like in Clojure)
(defrecord Student [name age score])

(defn print-student [s]
  (println (str (:name s) \" (\" (:age s) \"): \" (:score s))))

(defn -main []
  ;; Variables
  (let [count 5
        message \"Hello, Clojure!\"]

    ;; Vector of records
    (let [students [(->Student \"Alice\" 20 92.5)
                    (->Student \"Bob\"   22 85.0)
                    (->Student \"Charlie\" 21 78.3)]]

      ;; For loop over vector
      (println message)
      (println \"Students:\")
      (dorun (map print-student students))

      ;; Working with numeric vector
      (let [scores [88 95 70 82 91]
            sum (reduce + scores)]
        (println (str \"Average score: \" (float (/ sum (count scores)))))))))
"
        }
        "html" | "htm" => {
            "\
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>Hello, World!</title>
    <style>
        :root {
            color-scheme: light dark;
            font-family: system-ui, -apple-system, \"Segoe UI\", sans-serif;
        }
        body {
            max-width: 48rem;
            margin: 3rem auto;
            padding: 0 1.5rem;
            line-height: 1.6;
        }
        header {
            border-bottom: 1px solid color-mix(in srgb, currentColor 20%, transparent);
            margin-bottom: 2rem;
            padding-bottom: 1rem;
        }
        .card {
            border: 1px solid color-mix(in srgb, currentColor 20%, transparent);
            border-radius: 0.75rem;
            padding: 1.25rem;
        }
        button {
            border: 0;
            border-radius: 0.5rem;
            padding: 0.6rem 1rem;
            font: inherit;
            cursor: pointer;
        }
    </style>
</head>
<body>
    <header>
        <h1>Hello, World!</h1>
        <p>A starter page generated by ntc.</p>
    </header>
    <main>
        <section class=\"card\">
            <h2>Getting started</h2>
            <p>Edit this file and open it in a browser to see your changes.</p>
            <button id=\"greet\">Say hello</button>
            <p id=\"message\" aria-live=\"polite\"></p>
        </section>
    </main>
    <script>
        const button = document.getElementById(\"greet\");
        const message = document.getElementById(\"message\");
        button.addEventListener(\"click\", () => {
            message.textContent = \"Hello from ntc!\";
        });
    </script>
</body>
</html>
"
        }
        _ => "",
    }
}

pub fn init_super_file(path: &Path) -> Result<bool> {
    if path.exists() {
        return Ok(false);
    }
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let template = generate_super_template(ext);
    if template.is_empty() {
        return Ok(false);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, template)?;
    Ok(true)
}
