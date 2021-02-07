# semantic modulation:

- seperate compiler operations in individual modules/files.
- index modules by operation's type.
- key modules by operation's uniqueness | signature.

# semantic function:

- optimization: reused piece 
- segmentation: break if not constant statement needed to consume ressource.
- delegation: stateful execution

# nested dependecies:

- a { b( x(); y(); z() ) } // all dependeciens are in leaves' scopes.
- a ( x, y, z ) { b( x, y, z )} // all dependencies are in root scope.


## case: call-tree:
* 0 // static / main
  * 1
    * x, y
    * 3 ( x )
    * 4 ( x, y )
    * 5 ( y )
    * result ( 3, 4, 5 )

- 3 & 5 are out-of-order executed; then (logically) 4
- 3 & 4 depends on x to be instanciated.
- 4 & 5 depends on y to be instanciated
- result depends on 3, 4, 5.
- result does not need x nor y.
- x or y must be disposed after 4's execution.

## best way to scope ?
* ( ref, function ) = ( x, 3 ) | ( y, 5 );
* 4' = ref -> 4.bindByKey( ref.key, ref.value );
* function( ref.value );
* 4'( (!ref).value )
* (!function)( !ref.value )

# solution: run-time dependency injection:

- macro to express need of a ressource at compile-time
- will procure (first ?) module of that ressource in the parent list

* no global scope polution ( 0( x ), 1( y ){ 0(y) }, 2 { 1(z) }, z )
* no tight binding ( 0( x ){ 1( x ){ 2( x ) } } )

can't simply append module because of ressources lifetime; they have to be disposed at the common ancestor.