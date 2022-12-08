import scala.io.Source

object Day12 {

  final private val INPUT_DIR = "input/day12/"

  def traverse(
      adj: Map[String, List[String]],
      curr: String,
      path: List[String] = List.empty
  ): Int = {
    if (curr == "end") 1
    else if (path.filter(s => s == s.toLowerCase).contains(curr)) 0
    else adj(curr).map(t => traverse(adj, t, path :+ curr)).sum
  }

  def traverse2(
      adj: Map[String, List[String]],
      curr: String,
      path: List[String] = List.empty,
      allowRepeat: Boolean = true
  ): Int = {
    if (curr == "end") 1
    else if (path.filter(s => s == s.toLowerCase).contains(curr))
      if (allowRepeat && !List("start", "end").contains(curr))
        adj(curr).map(t => traverse2(adj, t, path, false)).sum
      else 0
    else adj(curr).map(t => traverse2(adj, t, path :+ curr, allowRepeat)).sum
  }

  def createAdjMap(input: String): Map[String, List[String]] =
    Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .map(e => (e.split("-")(0), e.split("-")(1)))
      .flatMap(e => List(e, (e._2, e._1)))
      .toList
      .groupBy(_._1)
      .mapValues(l => l.map(_._2))
      .toMap

  def part1(input: String): Int = {
    traverse(createAdjMap(input), "start")
  }

  def part2(input: String): Int = {
    traverse2(createAdjMap(input), "start")
  }
}
