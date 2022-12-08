import scala.io.Source
import scala.annotation.tailrec

class BingoBoard(boardNumbers: List[Int]) {

  final val WIN_LIST = List(
    List(0, 1, 2, 3, 4),
    List(5, 6, 7, 8, 9),
    List(10, 11, 12, 13, 14),
    List(15, 16, 17, 18, 19),
    List(20, 21, 22, 23, 24),
    List(0, 5, 10, 15, 20),
    List(1, 6, 11, 16, 21),
    List(2, 7, 12, 17, 22),
    List(3, 8, 13, 18, 23),
    List(4, 9, 14, 19, 24)
  )

  def isWin(pastNumbers: List[Int]): Boolean =
    WIN_LIST
      .map(_.map(e => pastNumbers.contains(boardNumbers(e))).reduce(_ && _))
      .reduce(_ || _)

  def unmarkedSum(pastNumbers: List[Int]): Int =
    boardNumbers.map(n => if (pastNumbers.contains(n)) 0 else n).sum

  def prnt(): Unit =
    println(boardNumbers)

}

object Day04 {

  final private val INPUT_DIR = "input/day04/"

  @tailrec
  def round(
      cBoards: List[BingoBoard],
      cNumbers: List[Int]
  ): Option[BingoBoard] =
    if (cBoards.isEmpty) None
    else if (cBoards.head.isWin(cNumbers)) Some(cBoards.head)
    else round(cBoards.tail, cNumbers)

  def part1(input: String): Int = {
    val lines = Source
      .fromFile(INPUT_DIR + input)
      .getLines()

    val numbers = lines.next().split(",").map(_.toInt)
    lines.next() // empty line
    val boards = lines
      .mkString("\n")
      .split("\n\n")
      .map(x => new BingoBoard(x.trim.split("\\s+").map(_.toInt).toList))

    @tailrec
    def play(currIdx: Int = 1): Int = {
      round(boards.toList, numbers.splitAt(currIdx)._1.toList) match {
        case Some(board) =>
          board.unmarkedSum(numbers.splitAt(currIdx)._1.toList) * numbers(
            currIdx - 1
          )
        case None =>
          if (currIdx < numbers.length) play(currIdx + 1)
          else throw new RuntimeException
      }
    }

    play()
  }

  def part2(input: String): Int = {
    val lines = Source
      .fromFile(INPUT_DIR + input)
      .getLines()

    val numbers = lines.next().split(",").map(_.toInt)
    lines.next() // empty line
    val boards = lines
      .mkString("\n")
      .split("\n\n")
      .map(x => new BingoBoard(x.trim.split("\\s+").map(_.toInt).toList))

    @tailrec
    def play(remainingBoards: List[BingoBoard], currIdx: Int = 1): Int = {
      if (currIdx > numbers.length) throw new RuntimeException
      round(remainingBoards.toList, numbers.splitAt(currIdx)._1.toList) match {
        case Some(board) =>
          if (remainingBoards.length == 1)
            board.unmarkedSum(numbers.splitAt(currIdx)._1.toList) * numbers(
              currIdx - 1
            )
          else {
            play(remainingBoards.filter(_ != board), currIdx)
          }
        case None =>
          if (currIdx < numbers.length) play(remainingBoards, currIdx + 1)
          else throw new RuntimeException
      }
    }

    play(boards.toList)
  }
}
