import Foundation

let PATH = "../input/day3"

struct Claim {
    var id: Int
    var left: Int
    var top: Int
    var width: Int
    var height: Int
}

func parseClaim(claim: String) -> Claim {
    let regex = try? NSRegularExpression(pattern: "([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)")
    if let match = regex?.firstMatch(in: claim, options: [], range: NSRange(location: 0, length: claim.count)) {
        if let idRange = Range(match.range(at: 1), in: claim),
            let leftRange = Range(match.range(at: 2), in: claim),
            let topRange = Range(match.range(at: 3), in: claim),
            let widthRange = Range(match.range(at: 4), in: claim),
            let heightRange = Range(match.range(at: 5), in: claim) {
                return Claim(
                    id: Int(claim[idRange])!,
                    left: Int(claim[leftRange])!,
                    top: Int(claim[topRange])!,
                    width: Int(claim[widthRange])!,
                    height: Int(claim[heightRange])!
                )
            }
    }

    return Claim(id: 0, left: 0, top: 0, width: 0, height: 0)
}

func solution() {
    let file = try! String(contentsOfFile: PATH)

    let claims:[String] = file.trimmingCharacters(in: .whitespacesAndNewlines).components(separatedBy: .newlines)

    let resultClaims = claims.map { claim in
        parseClaim(claim: claim)
    }

    var fabricGrid = Array(repeating: Array(repeating: 0, count: 1000), count: 1000)

    var result = 0
    for claim in resultClaims {
        for x in stride(from: claim.top, to: claim.top + claim.height, by: 1) {
            for y in stride(from: claim.left, to: claim.left + claim.width, by: 1) {
                fabricGrid[x][y] += 1
                if fabricGrid[x][y] > 1 { result += 1 }
            }
        }
    }

    // Part 1
    print("Part 1 result: \(result)")

    var nonoverlap = true
    var result2 = 0
    for claim in resultClaims {
        for x in stride(from: claim.top, to: claim.top + claim.height, by: 1) {
            for y in stride(from: claim.left, to: claim.left + claim.width, by: 1) {
                if fabricGrid[x][y] > 1 { nonoverlap = false }
            }
        }

        if nonoverlap {
            result2 = claim.id
            break
        } else { nonoverlap = true }
    }

    // Part 2
    print("Part 2 result: \(result2)")
}

solution()
