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

def updateLayers(layers: Map[Int, Scanner]): Map[Int, Scanner] = {
    layers.map { case (layerNum, scanner) =>
        (layerNum, updateScannerPosition(scanner))
    }.withDefaultValue(null)
}

val firewallSize = layers.last._1
def tryMoveThroughFirewall(layers: Map[Int, Scanner], layerNum: Int): Boolean = {
    if (layerNum > firewallSize) {
        true
    } else {
        layers(layerNum) match {
            case Scanner(0, _, _) => false
            case _ => tryMoveThroughFirewall(updateLayers(layers), layerNum + 1)
        }
    }
}

def moveThroughFirewall(layers: Map[Int, Scanner], delay: Int): Int = {
    if (tryMoveThroughFirewall(layers, 0)) {
        delay
    } else {
        moveThroughFirewall(updateLayers(layers), delay + 1)
    }
}

println(moveThroughFirewall(layers, 0))