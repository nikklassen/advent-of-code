def findDistance(steps) {
    stepCounts = [
        'ne': 0,
        'nw': 0,
        'se': 0,
        'sw': 0,
        'n': 0,
        's': 0,
    ]
    for (step in steps) {
        stepCounts[step] += 1
    }

    def changed = true
    while (changed) {
        changed = false

        def reductionPairs = [
            ['n', 'sw', 'nw'],
            ['n', 'se', 'ne'],
            ['s', 'nw', 'sw'],
            ['s', 'ne', 'se'],
            ['ne', 'nw', 'n'],
            ['se', 'sw', 's'],
            ['ne', 'sw'],
            ['nw', 'se'],
            ['n', 's'],
        ]
        reductionPairs.forEach { pair ->
            def common = Math.min(stepCounts[pair[0]], stepCounts[pair[1]])
            if (common > 0) {
                changed = true
            }
            stepCounts[pair[0]] -= common
            stepCounts[pair[1]] -= common
            if (pair[2]) {
                stepCounts[pair[2]] += common
            }
        }
    }

    totalSteps = 0
    stepCounts.values().forEach { v -> totalSteps += v }
    return totalSteps
}

new File('input').eachLine { line ->
    def steps = line.split(',')
    println findDistance(steps)
}