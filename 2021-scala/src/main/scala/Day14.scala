import scala.io.Source

object Day14:

  final private val INPUT_DIR = "input/day14/"

  def parseInput(input: String): (String, Map[String, List[String]]) =
    val lines = Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .toList
    val start = lines(0)
      .strip()
    val map = lines
      .splitAt(2)
      ._2
      .map(l =>
        val sp = l.split(" -> ")
        (sp(0), List(s"${sp(0)(0)}${sp(1)}", s"${sp(1)}${sp(0)(1)}"))
      )
      .toMap
    (start, map)

  def process(
      initialState: List[(String, BigInt)],
      map: Map[String, List[String]],
      ends: String,
      cycles: Int,
  ): Map[Char, BigInt] =
    Range(0, cycles)
      .foldLeft(initialState)((s, i) =>
        s.flatMap(pn => map(pn._1).map((_, pn._2)))
          .groupBy(_._1)
          .mapValues(_.foldLeft(BigInt(0))(_ + _._2))
          .toList
      )
      .flatMap(pn => List((pn._1(0), pn._2), (pn._1(1), pn._2)))
      .groupBy(_._1)
      .mapValues(_.foldLeft(BigInt(0))(_ + _._2))
      .map(cn => if ends.contains(cn._1) then (cn._1, cn._2 + 1) else cn)
      .map(cn => (cn._1, cn._2 / 2))
      .toMap

  def getState(start: String): List[(String, BigInt)] =
    start
      .sliding(2)
      .toList
      .groupBy(identity)
      .mapValues(v => BigInt(v.length))
      .toList

  def getResult(input: String, cycles: Int): BigInt =
    val (start, map) = parseInput(input)
    val state = getState(start)
    val endState =
      process(state, map, s"${start(0)}${start(start.length - 1)}", cycles)
    endState.values.max - endState.values.min

  def part1(input: String): BigInt =
    getResult(input, 10)

  def part2(input: String): BigInt =
    getResult(input, 40)
