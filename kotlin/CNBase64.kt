private const val CODE = "锟斤拷烫"
fun UByte.encode(sb: StringBuilder) {
    var i = toInt()
    sb.append(CODE[i and 3])
    i = i ushr 2
    sb.append(CODE[i and 3])
    i = i ushr 2
    sb.append(CODE[i and 3])
    i = i ushr 2
    sb.append(CODE[i and 3])
}
fun Byte.encode(sb: StringBuilder) = toUByte().encode(sb)
fun ByteArray.encode(sb: StringBuilder) = forEach { it.encode(sb) }
fun String.encode(sb: StringBuilder) = toByteArray().encode(sb)
fun Char.decode() = when (this) {
    '锟' -> 0
    '斤' -> 1
    '拷' -> 2
    '烫' -> 3
    else -> error("Invalid Character $this in CN-Base4 Code")
}
fun String.decode(): ByteArray {
    require(length and 3 == 0) { "Malformed Length of CN-Base4 Code" }
    return chunked(4).map { s ->
        val u = s.map { c -> c.decode() }
        (u[0] or (u[1] shl 2) or (u[2] shl 4) or (u[3] shl 6)).toByte()
    }.toByteArray()
}


fun main() {
    val sb = StringBuilder()
    "114514".encode(sb)
    require(sb.toString() == "斤锟烫锟斤锟烫锟锟斤烫锟斤斤烫锟斤锟烫锟锟斤烫锟")
    require(String(sb.toString().decode()) == "114514")
}
