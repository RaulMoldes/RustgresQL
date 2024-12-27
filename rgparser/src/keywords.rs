
// List of SQL functions
pub const SQL_FUNCTIONS: &[&str] = &[
    "abs", "avg", "ceil", "ceiling", "coalesce", "count", "covar_pop", "covar_samp", "exp", "extract", "floor",
    "lag", "lead", "ln", "max", "min", "round", "sum", "variance",
];


pub const SQL_DATATYPES: &[&str] = &[
    "bigint", "binary", "bit", "blob", "bool", "boolean", "char", "character", "date", "datetime", "decimal",
    "double", "enum", "float", "int", "integer", "interval", "long", "longblob", "longtext", "mediumblob",
    "mediumint", "mediumtext", "numeric", "real", "set", "smallint", "text", "time", "timestamp", "tinyblob",
    "tinyint", "tinytext", "varbinary", "varchar", "year"

];

// List of SQL CLAUSES
pub const SQL_KEYWORDS: &[&str] = &[
    "select", "from", "where", "group", "having", "order", "by", "join", "inner", "left", "right", "outer", "on", "as", "with", "update", "insert", "values", "delete", "set", "union", "cross", "limit", "offset", "distinct", "case", "when", "then", "else", "end", "exists", "not", "in", "like", "ilike", "similar", "between", "is", "null", "and", "or", "xor", "any", "all", "some", "exists", "not", "in", "like", "ilike", "similar", "between", "is", "null", "and", "or", "xor", "any", "all", "some", "exists", "not", "in", "all", "explain", "analyze", "create", "alter", "drop", "truncate", "rename", "comment", "set", "grant", "revoke", "begin", "commit", "rollback", "savepoint", "lock", "unlock", "call", "do", "handler", "declare", "continue", "leave", "iterate", "repeat", "return", "while", "if", "else", "elseif", "then", "case", "when", "end", "signal", "resignal", "get", "diagnostics", "condition", "declare", "handler", "for", "loop", "while", "repeat", "until", "leave", "iterate", "open", "close", "fetch", "using", "into", "out", "inout", "as", "begin", "end", "exit", "execute", "prepare", "deallocate", "analyze", "backup", "check", "checksum", "optimize", "repair", "restore", "show", "kill", "lock", "unlock", "flush", "purge", "reset", "shutdown", "start", "stop", "grant", "revoke", "set", "show", "use", "analyze", "check", "checksum", "optimize", "repair", "restore", "show", "kill", "lock", "unlock", "flush", "purge", "reset", "shutdown", "start", "stop", "grant", "revoke", "set", "show", "use", "analyze", "check", "checksum", "optimize", "repair", "restore", "show", "kill", "lock", "unlock", "flush", "purge", "reset", "shutdown", "start", "stop"
];


// List of SQL entities
pub const SQL_ENTITIES: &[&str] = &[
    "table", "constraint", "view", "procedure","function", "trigger", "index", "database", "schema", "column", "row",
];

// List of SQL punctuation
pub const PUNCTUATION: &[char] = &[
    ';', ',', '(', ')', '{', '}', '[', ']', ':', '.'
];

// List of SQL str delimiters
pub const LITERAL_DELIMITERS: &[char] = &['\'', '\"'];

// List of SQL operators
pub const OPERATORS: &[char] = &[
    '+', '-', '*', '/', '%', '<', '>', '!', '^', '&', '|', '~','=',
];