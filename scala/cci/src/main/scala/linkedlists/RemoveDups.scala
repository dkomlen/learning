package linkedlists

import common.{Solution, Solver}

import scala.annotation.tailrec
import scala.collection.mutable
import scala.collection.mutable.ListBuffer

object RemoveDups extends Solution[List[Int], List[Int]] {

  solvers.append(Solver("N-recursive", sol1(Set[Int](), ListBuffer())))
  solvers.append(Solver("N-iterative-flatten", sol2))

  @tailrec
  def sol1(set: Set[Int], acc: ListBuffer[Int])(input: List[Int]): List[Int] = input match {
    case Nil => {
      val sol = acc.toList
      acc.clear()
      sol
    }
    case x :: xs if set.contains(x) => sol1(set, acc)(xs)
    case x :: xs => {
      acc.append(x)
      sol1(set + x, acc)(xs)
    }
  }

  def sol2(input: List[Int]): List[Int] = {
    val set = mutable.SortedSet[Int]()
    (for (x <- input) yield {
      val ret = set.add(x) match {
        case true => Some(x)
        case false => None
      }
      ret
    }).flatten
  }
}
