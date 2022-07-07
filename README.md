# TheRusticanAdventures
This game will be a text adventure written in Rust

## Map
The map is a hold very simple. The map is made by a grid. Every cell has different possible values. Every cell is seperated by ;

They are coded as Hex Value.

* 0x0X:     **Way**
    * 0x00:     with stone
    * 0x01:     with grass on it
    * 0x02:

* 0x1X:     **Tree**
    * 0x10:     birch
    * 0x11:     oak
    * 0x12:     pine tree
    * 0x13:     cedar
    * 0x14:     palm
    * ...

* 0x2X:     **Wall**
    * 0x20:     brick
    * 0x21:     loam
    * 0x22:     cement
    * 0x23:     concrete
    * 0x24:     wood
    * ...
* **...**

* 0xFF:     **point of starting**