import unittest
import Vcypher


class TestVcypher(unittest.TestCase):
    def test_output(self):
        self.assertEqual(Vcypher.vcypher("BFC"), "81156121701")
        self.assertEqual(Vcypher.vcypher("*()"), "8123614147318")

    def test_edge_cases_output(self):
        self.assertEqual(Vcypher.vcypher(""), "")

    def test_non_english_letters_output(self):
        self.assertEqual(Vcypher.vcypher("ľ"), "623")
        self.assertEqual(Vcypher.vcypher("š"), "415")
        self.assertEqual(Vcypher.vcypher("č"), "445")
        self.assertEqual(Vcypher.vcypher("ť"), "5134")
        self.assertEqual(Vcypher.vcypher("ž"), "712")
        self.assertEqual(Vcypher.vcypher("ý"), "71")
        self.assertEqual(Vcypher.vcypher("á"), "44")
        self.assertEqual(Vcypher.vcypher("í"), "612")
        self.assertEqual(Vcypher.vcypher("é"), "513")


if __name__ == '__main__':
    unittest.main()
