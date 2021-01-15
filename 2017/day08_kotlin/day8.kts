import java.io.File
import java.lang.Integer.max

data class Instruction(var reg: String, var incDec: String, var op: Int, var comparisonReg: String, var comparison: String, var predicate: Int)

fun main(maxReg: Boolean) {
    val instructions = arrayListOf<Instruction>()
    val registers = mutableMapOf<String, Int>()
    File("input").forEachLine {
        val match = Regex("([a-z]+) (inc|dec) (-?\\d+) if ([a-z]+) (\\S+) (-?\\d+)").find(it) ?: return@forEachLine
        val register = match.groupValues[1]
        val comparisonReg = match.groupValues[4]
        registers[register] = 0
        registers[comparisonReg] = 0
        val instruction = Instruction(register, match.groupValues[2], match.groupValues[3].toInt(), comparisonReg, match.groupValues[5], match.groupValues[6].toInt())
        instructions.add(instruction)
    }

    var maxVal = Int.MIN_VALUE
    instructions.forEach {
        val comparisonRegValue = registers.getOrDefault(it.comparisonReg, 0)
        val doOp = when {
            it.comparison == "==" -> comparisonRegValue == it.predicate
            it.comparison == "!=" -> comparisonRegValue != it.predicate
            it.comparison == ">=" -> comparisonRegValue >= it.predicate
            it.comparison == ">" -> comparisonRegValue > it.predicate
            it.comparison == "<=" -> comparisonRegValue <= it.predicate
            it.comparison == "<" -> comparisonRegValue < it.predicate
            else -> false
        }
        if (!doOp) {
            return@forEach
        }

        var regValue = registers.getOrDefault(it.reg, 0)
        if (it.incDec == "inc") {
            regValue += it.op
        } else {
            regValue -= it.op
        }
        maxVal = max(maxVal, regValue)
        registers[it.reg] = regValue
    }

    if (maxReg) {
        // Part 1
        println(registers.maxBy { (_, v) -> v })
    } else {
        // Part 2
        println(maxVal)
    }
}

main(false)
