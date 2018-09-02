package arrays

import arrays.IsUnique.solve
import common.Input
import org.scalatest.Matchers

class IsUniqueTest extends org.scalatest.FlatSpec with Matchers {

  val maxString = (Char.MinValue to Char.MaxValue).filter(_.isLetterOrDigit).mkString("")

  "isUnique" should "return true for empty string" in {
    solve(Input("empty", "")) should be(true)
  }

  it should "return true for positive cases" in {
    solve(Input("true-simple", "abcdefg")) should be(true)
    solve(Input("true-maxstring", maxString)) should be(true)
  }

  it should "return false for negative cases" in {
    solve(Input("false-simple", "abcdefgc")) should be(false)
    solve(Input("false-simple2", "abcdefg" * 20)) should be(false)
    solve(Input("false-maxstring", maxString + "f")) should be(false)
  }
}
