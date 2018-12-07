import Foundation

let PATH = "../input/day2"

func hasCountOfSameLetter(input: String, count: Int) -> Bool {
    var exactCount = 0
    var resultMap: [Character: Int] = [:]
    for char in input {
        if let charCount = resultMap[char] {
            let newCount = charCount + 1
            if newCount == count {
                exactCount += 1
            } else if newCount == count + 1 {
                exactCount -= 1
            }

            resultMap[char] = newCount
        } else {
            resultMap[char] = 1
        }
    }

    return exactCount > 0
}

func commonString(word1: String, word2: String) -> String {
    var matchCount = 0
    var resultArray: [Character] = []
    for (char1, char2) in zip(word1, word2) {
        if char1 == char2 {
            matchCount += 1
            resultArray.append(char1)
        }
    }

    return String(resultArray)
}

func solution() {
    let file = try! String(contentsOfFile: PATH)

    let ids:[String] = file.trimmingCharacters(in: .whitespacesAndNewlines).components(separatedBy: .newlines)

    var count2s = 0
    var count3s = 0

    ids.forEach { id in
        if hasCountOfSameLetter(input: id, count: 2) {
            count2s += 1
        }

        if hasCountOfSameLetter(input: id, count: 3) {
            count3s += 1
        }
    }

    // Part1
    print("Part 1 result: \(count2s * count3s)")

    var part2Result = ""
    for index1 in stride(from: 0, to: ids.count, by: 1) {
        let id1 = ids[index1]
        for index2 in stride(from: index1 + 1, to: ids.count, by: 1) {
            let id2 = ids[index2]
            let result = commonString(word1: id1, word2: id2)

            if result.count + 1 == id2.count {
                part2Result = result
                break
            }
        }

        if part2Result != "" {
            break
        }
    }

    // Part2
    print("Part 2 result: \(part2Result)")
}

solution()
