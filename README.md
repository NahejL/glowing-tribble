<!-- <img src="repo/splash.png" alt="Amethyst 2D starter template" /> -->
a game!


# issue of dependencies
## segmentation: 
- 0
  - 1
    - 4
    - 5
      - 12
        - 15
      - 13
        - 16
      - 14
    - 6
  - 2
    - 7
    - 8
  - 3
    - 10
    - 11

* a structure may depend on some data to be instanciated ( 0, 1, 5, 12, 13, 2, 3 are dependant )
* this data may have it's own dependencies ( 1, 5, 2, 3 are depended/parents and dependant/children )
* a data's lifetime is no longer than it's longest living dependant/parent instanciation ( 1 sould not be kept longer than 0 )
* a parent should not depend of a child's dependency ( thightly coppled ) since child's dependencies may change

## solution:
compile time dependency injection ?
* each struture defines its dependencies (with code?)
* a dependency is placed in optimal scope ( common parent | joined parent )
* structures inbetween are passed a "context" to carry children's dependencies  

