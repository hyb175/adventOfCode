import Foundation

let PATH = "input"

func solution() {
  // Part 1
  let file = try! String(contentsOfFile: PATH)

  let numbers: [String] = file.trimmingCharacters(in: .whitespacesAndNewlines).components(separatedBy: .newlines)

  let result = numbers.compactMap {Int($0)}.reduce(0) { sum, number in sum + (number / 3 - 2) }

  print("[Day1] Part1 result is: \(result)")
  // Part 2
}

solution()
