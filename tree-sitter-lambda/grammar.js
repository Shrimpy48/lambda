module.exports = grammar({
  name: 'lambda',

  rules: {
    source: $ => $._term,

    _term: $ => choice(
        $.identifier,
        $.abstraction,
        $.application,
        seq('(', $._term, ')'),
        ),

    identifier: $ => /[a-z_][a-z0-9_]*/,

    application: $ => prec.left(1, seq(
        field('lhs', $._term),
        field('rhs', $._term)
    )),

    abstraction: $ => seq(
        choice("\\", "λ"),
        field('bound', $.identifier),
        ".",
        field('body', $._term)
    ),

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
