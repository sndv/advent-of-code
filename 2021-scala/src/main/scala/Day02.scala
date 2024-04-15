import scala.io.Source

object Day02:

  final private val INPUT_DIR = "input/day02/"

  def part1(input: String): Int =
    val lines = Source.fromFile(INPUT_DIR + input).getLines().toList
    def extract(name: String): Int =
      lines
        .filter(_.contains(name))
        .map(_.split(" ")(1).toInt)
        .sum
    extract("forward") * (extract("down") - extract("up"))

  def part2(input: String): Int =
    def calc(
        aim: Int,
        hor: Int,
        depth: Int,
        remLines: Iterable[String],
    ): Tuple3[Int, Int, Int] =
      val (aimL, rest) = remLines.span(!_.contains("forward"))
      if rest.isEmpty then (aim, hor, depth)
      else {
        val currAim = aim + aimL
          .map(_ match
            case l if l.contains("down") => l.split(" ")(1).toInt
            case l if l.contains("up")   => -l.split(" ")(1).toInt,
          )
          .sum
        val rSplit = rest.splitAt(1)
        val fwd = rSplit._1.toList(0).split(" ")(1).toInt
        calc(currAim, hor + fwd, depth + (currAim * fwd), rSplit._2)
      }

    val lines = Source.fromFile(INPUT_DIR + input).getLines()
    val (_, hor, depth) = calc(0, 0, 0, lines.toIterable)
    hor * depth
