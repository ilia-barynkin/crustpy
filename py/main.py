from cmsis_svd.parser import SVDParser
from sys import argv

parser = SVDParser.for_xml_file(argv[1])

for peripheral in parser.get_device().peripherals:
    print(peripheral.name)