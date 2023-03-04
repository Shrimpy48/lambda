module.exports = grammar({
  name: 'lambda',

  rules: {
    source: $ => $._term,

    _term: $ => choice(
        $.variable,
        $.sort,
        $.abstraction,
        $.application,
        $.product,
        seq('(', $._term, ')'),
        seq('[', $._term, ']'),
        ),

    sort: $ => choice('*', '◻'),

    // Exclude lambda and uppercase pi
    variable: $ => /[a-zA-Zα-κμ-ωΑ-ΚΜ-ΟΡ-Ω_][a-zA-Zα-κμ-ωΑ-ΚΜ-ΟΡ-Ω0-9_]*'*/,

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

    product: $ => choice(
        seq(
            choice("TT", "Π", "∀"),
            field('input_name', $.variable),
            optional(seq(choice(":", "::"), field('input', $._term))),
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
