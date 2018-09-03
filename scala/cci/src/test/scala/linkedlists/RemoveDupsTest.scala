package linkedlists

import common.Input
import org.scalatest.Matchers

class RemoveDupsTest extends org.scalatest.FlatSpec with Matchers {

  "RemoveDups" should "remove duplicates" in {
    RemoveDups.solve(Input("small", List(8,1,3,2,2,3,4,1,5,6,7,8))) should be(List(8,1,3,2,2,3,4,1,5,6,7,8).distinct)
    RemoveDups.solve(Input("single-element-large", List.fill(100000)(0))) should be(List(0))
    RemoveDups.solve(Input("all-distinct-large", (0 to 100000).toList)) should be((0 to 100000).toList)
  }
}
