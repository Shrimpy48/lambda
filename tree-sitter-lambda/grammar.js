module.exports = grammar({
  name: 'lambda',

  rules: {
    source: $ => $._term,

    _term: $ => choice(
        $.variable,
        $.sort,
        $.constant,
        $.abstraction,
        $.application,
        $.function,
        seq('(', $._term, ')'),
        seq('[', $._term, ']'),
        ),

    sort: $ => choice('*', '◻'),

    variable: $ => /[a-zA-Z_][a-zA-Z0-9_]*'*/,

    constant: $ => choice("i", "ι"),

    application: $ => prec.left(1, seq(
        field('lhs', $._term),
        field('rhs', $._term)
    )),

    abstraction: $ => seq(
        choice("\\", "λ", "Λ"),
        field('bound', $.variable),
        optional(seq(choice(":", "::"), field('bind_type', $._term))),
        ".",
        field('body', $._term)
    ),

    function: $ => choice(
        seq(
            choice("TT", "Π", "∀"),
            optional(seq(field('input_name', $.variable), /::?/)), 
            field('input', $._term),
            ".",
            field('output', $._term)
        ),
        prec.right(1, seq(
            field('input', $._term), 
            choice("->", "→", "⟶"), 
            field('output', $._term)
        )
    )),

    block_comment: $ => seq(
        '/*',
        repeat(/./),
        prec(100, '*/')
    ),

    line_comment: $ => seq(
        '//',
        repeat(token.immediate(/[^\r\n]/)),
        token.immediate(/\r?\n/)
    )
  },

  extras: $ => [
    /\s+/,
    $.line_comment,
    $.block_comment
  ],

});
