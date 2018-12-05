import Foundation

let PATH = "../../../input/day1"

func solution() {
    let file = try! String(contentsOfFile: PATH)

    let numbers:[String] = file.trimmingCharacters(in: .whitespacesAndNewlines).components(separatedBy: .newlines)

    var result = 0
    numbers.forEach { number in
        let index = number.index(number.startIndex, offsetBy: 1)
        result += (number[number.startIndex] == "+" ? 1 : (-1)) * Int(String(number[index..<number.endIndex]))!
    }

    // Part 1:
    print("Part1 result is: \(result)")

    var dict = ["0": true]
    result = 0
    var occurredTwice = false

    while(true) {
        for number in numbers {
            let index = number.index(number.startIndex, offsetBy: 1)
            result += (number[number.startIndex] == "+" ? 1 : (-1)) * Int(String(number[index..<number.endIndex]))!

            if let occurred = dict[String(result)] {
                if occurred {
                    occurredTwice = true
                    break
                }
            }

            dict[String(result)] = true
        }

        if occurredTwice { break }
    }

    // Part 2:
    print("Part2 result is: \(result)")
}

solution()
