object Main extends App {

  def exec(f: (String) => Any, input: String, expected: String = ""): String = {
    val start = System.currentTimeMillis()
    val result = s"${f(input)}"
    val duration = System.currentTimeMillis() - start
    val incorrect =
      if (expected != "" && result != expected) " (incorrect!)" else ""
    f"${result}${incorrect} (${duration.toDouble / 1000}%.3fs)"
  }

  println(s"01-1 ex: ${exec(Day01.part1, "input-ex", "7")}")
  println(s"01-1 in: ${exec(Day01.part1, "input", "1477")}")
  println(s"01-2 ex: ${exec(Day01.part2, "input-ex", "5")}")
  println(s"01-2 in: ${exec(Day01.part2, "input", "1523")}")
  println()
  println(s"02-1 ex: ${exec(Day02.part1, "input-ex", "150")}")
  println(s"02-1 in: ${exec(Day02.part1, "input", "1524750")}")
  println(s"02-2 ex: ${exec(Day02.part2, "input-ex", "900")}")
  println(s"02-2 in: ${exec(Day02.part2, "input", "1592426537")}")
  println()
  println(s"03-1 ex: ${exec(Day03.part1, "input-ex", "198")}")
  println(s"03-1 in: ${exec(Day03.part1, "input", "2498354")}")
  println(s"03-2 ex: ${exec(Day03.part2, "input-ex", "230")}")
  println(s"03-2 in: ${exec(Day03.part2, "input", "3277956")}")
  println()
  println(s"04-1 ex: ${exec(Day04.part1, "input-ex", "4512")}")
  println(s"04-1 in: ${exec(Day04.part1, "input", "33348")}")
  println(s"04-2 ex: ${exec(Day04.part2, "input-ex", "1924")}")
  println(s"04-2 in: ${exec(Day04.part2, "input", "8112")}")
  println()
  println(s"05-1 ex: ${exec(Day05.part1, "input-ex", "5")}")
  println(s"05-1 in: ${exec(Day05.part1, "input", "6267")}")
  println(s"05-2 ex: ${exec(Day05.part2, "input-ex", "12")}")
  println(s"05-2 in: ${exec(Day05.part2, "input", "20196")}")
  println()
  println(s"06-1 ex: ${exec(Day06.part1, "input-ex", "5934")}")
  println(s"06-1 in: ${exec(Day06.part1, "input", "385391")}")
  println(s"06-2 ex: ${exec(Day06.part2, "input-ex", "26984457539")}")
  println(s"06-2 in: ${exec(Day06.part2, "input", "1728611055389")}")
  println()
  println(s"07-1 ex: ${exec(Day07.part1, "input-ex", "37")}")
  println(s"07-1 in: ${exec(Day07.part1, "input", "342534")}")
  println(s"07-2 ex: ${exec(Day07.part2, "input-ex", "168")}")
  println(s"07-2 in: ${exec(Day07.part2, "input", "94004208")}")
  println()
  println(s"08-1 ex: ${exec(Day08.part1, "input-ex", "26")}")
  println(s"08-1 in: ${exec(Day08.part1, "input", "310")}")
  println(s"08-2 ex: ${exec(Day08.part2, "input-ex", "61229")}")
  println(s"08-2 in: ${exec(Day08.part2, "input", "915941")}")
  println()
  println(s"09-1 ex: ${exec(Day09.part1, "input-ex", "15")}")
  println(s"09-1 in: ${exec(Day09.part1, "input", "607")}")
  println(s"09-2 ex: ${exec(Day09.part2, "input-ex", "1134")}")
  println(s"09-2 in: ${exec(Day09.part2, "input", "900864")}")
  println()
  println(s"10-1 ex: ${exec(Day10.part1, "input-ex", "26397")}")
  println(s"10-1 in: ${exec(Day10.part1, "input", "268845")}")
  println(s"10-2 ex: ${exec(Day10.part2, "input-ex", "288957")}")
  println(s"10-2 in: ${exec(Day10.part2, "input", "4038824534")}")
  println()
  println(s"11-1 ex: ${exec(Day11.part1, "input-ex", "1656")}")
  println(s"11-1 in: ${exec(Day11.part1, "input", "1691")}")
  println(s"11-2 ex: ${exec(Day11.part2, "input-ex", "195")}")
  println(s"11-2 in: ${exec(Day11.part2, "input", "216")}")

  // println()
  // println(s"06-2 (slow): ${exec(Day06.part2slow, "input", "1728611055389")}")

  println("\nDone.")

}
