import easy.TwoSum
import spock.lang.Specification

class TwoSumTest extends Specification {
    def "TwoSum should return an array of two indices whose sum equal to the given number"() {

        when: "run the sum function"
        def out = TwoSum.twoSum(numberSet as int[], sum)

        then:
        out[0] == result[0]
        out[1] == result[1]

        where:
        sum | numberSet           | result
        1   | [2, 8, 123, -3, -1] | [0, 4]
    }
}