import unittest

class Element:
    pass

class Collection:
    def __init__(self):
        self.elements = []
    
    def add(self, element=None, collection=None):
        if collection is not None:
            self.elements.extend(collection.elements)
        else:
            self.elements.append(element)


class TestCase(unittest.TestCase):

    def setUp(self):
        unittest.TestCase.setUp(self)

    def tearDown(self):
        unittest.TestCase.tearDown(self)

    def test(self):
        e = Element()
        c = Collection()
        c.add(e)
        c1 = Collection()
        c1.elements.append(Element())
        c.add(collection=c1)
        self.assertTrue(len(c.elements) == 2)

if __name__ == '__main__':
    unittest.main()