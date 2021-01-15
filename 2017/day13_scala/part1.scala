case class Scanner(position: Int, movingForward: Boolean, range: Int)

val layers = io.Source.fromFile("input").getLines.foldLeft(Map[Int, Scanner]().withDefaultValue(null)) { (edges, line) =>
    val parts = line.split(": ")
    edges + (parts(0).toInt -> Scanner(0, true, parts(1).toInt))
}

def updateScannerPosition(s: Scanner): Scanner = {
    val Scanner(position, movingForward, range) = s
    if (movingForward && position == range - 1) {
        Scanner(position - 1, false, range)
    } else if (!movingForward && position == 0) {
        Scanner(position + 1, true, range)
    } else {
        val newPosition = if (movingForward) { position + 1 } else { position - 1}
        Scanner(newPosition, movingForward, range)
    }
}

val firewallSize = layers.last._1
val (_, severity) = (0 to firewallSize).foldLeft((layers, 0)) { (acc, layerNum) =>
    val (layers, severity) = acc
    val newLayers = layers.map { case (layerNum, scanner) =>
        (layerNum, updateScannerPosition(scanner))
    }.withDefaultValue(null)

    layers(layerNum) match {
        case null => (newLayers, severity)
        case Scanner(position, _, range) => {
            val newSeverity = if (position == 0) {
                severity + range * layerNum
            } else {
                severity
            }
            (newLayers, newSeverity)
        }
    }
}

println(severity)