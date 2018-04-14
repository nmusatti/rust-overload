import math
import unittest

class Point:
    def __init__(self, x=None, y=None, r=None, t=None):
        if x is not None and y is not None:
            self.x = x
            self.y = y
        else:
            self.x = r * math.cos(t)
            self.y = r * math.sin(t)
    
    def __eq__(self, other):
        return self.x == other.x and self.y == other.y


class TestCase(unittest.TestCase):

    def setUp(self):
        unittest.TestCase.setUp(self)

    def tearDown(self):
        unittest.TestCase.tearDown(self)

    def test(self):
        p1 = Point(x=2.0, y=0.0)
        p2 = Point(r=2.0, t=0.0)
        self.assertTrue(p1 == p2)

if __name__ == '__main__':
    unittest.main()