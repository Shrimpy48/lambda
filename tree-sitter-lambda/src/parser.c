#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 36
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 30
#define ALIAS_COUNT 0
#define TOKEN_COUNT 19
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 5
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 4

enum {
  anon_sym_LPAREN = 1,
  anon_sym_RPAREN = 2,
  sym_identifier = 3,
  anon_sym_BSLASH = 4,
  anon_sym_ = 5,
  anon_sym_COLON = 6,
  anon_sym_DOT = 7,
  anon_sym_i = 8,
  anon_sym_2 = 9,
  anon_sym_DASH_GT = 10,
  anon_sym_3 = 11,
  anon_sym_4 = 12,
  anon_sym_SLASH_STAR = 13,
  aux_sym_block_comment_token1 = 14,
  anon_sym_STAR_SLASH = 15,
  anon_sym_SLASH_SLASH = 16,
  aux_sym_line_comment_token1 = 17,
  aux_sym_line_comment_token2 = 18,
  sym_source = 19,
  sym__term = 20,
  sym_application = 21,
  sym_abstraction = 22,
  sym__type = 23,
  sym_base_type = 24,
  sym_function_type = 25,
  sym_block_comment = 26,
  sym_line_comment = 27,
  aux_sym_block_comment_repeat1 = 28,
  aux_sym_line_comment_repeat1 = 29,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_identifier] = "identifier",
  [anon_sym_BSLASH] = "\\",
  [anon_sym_] = "λ",
  [anon_sym_COLON] = ":",
  [anon_sym_DOT] = ".",
  [anon_sym_i] = "i",
  [anon_sym_2] = "ι",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_3] = "→",
  [anon_sym_4] = "⟶",
  [anon_sym_SLASH_STAR] = "/*",
  [aux_sym_block_comment_token1] = "block_comment_token1",
  [anon_sym_STAR_SLASH] = "*/",
  [anon_sym_SLASH_SLASH] = "//",
  [aux_sym_line_comment_token1] = "line_comment_token1",
  [aux_sym_line_comment_token2] = "line_comment_token2",
  [sym_source] = "source",
  [sym__term] = "_term",
  [sym_application] = "application",
  [sym_abstraction] = "abstraction",
  [sym__type] = "_type",
  [sym_base_type] = "base_type",
  [sym_function_type] = "function_type",
  [sym_block_comment] = "block_comment",
  [sym_line_comment] = "line_comment",
  [aux_sym_block_comment_repeat1] = "block_comment_repeat1",
  [aux_sym_line_comment_repeat1] = "line_comment_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_identifier] = sym_identifier,
  [anon_sym_BSLASH] = anon_sym_BSLASH,
  [anon_sym_] = anon_sym_,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_i] = anon_sym_i,
  [anon_sym_2] = anon_sym_2,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_3] = anon_sym_3,
  [anon_sym_4] = anon_sym_4,
  [anon_sym_SLASH_STAR] = anon_sym_SLASH_STAR,
  [aux_sym_block_comment_token1] = aux_sym_block_comment_token1,
  [anon_sym_STAR_SLASH] = anon_sym_STAR_SLASH,
  [anon_sym_SLASH_SLASH] = anon_sym_SLASH_SLASH,
  [aux_sym_line_comment_token1] = aux_sym_line_comment_token1,
  [aux_sym_line_comment_token2] = aux_sym_line_comment_token2,
  [sym_source] = sym_source,
  [sym__term] = sym__term,
  [sym_application] = sym_application,
  [sym_abstraction] = sym_abstraction,
  [sym__type] = sym__type,
  [sym_base_type] = sym_base_type,
  [sym_function_type] = sym_function_type,
  [sym_block_comment] = sym_block_comment,
  [sym_line_comment] = sym_line_comment,
  [aux_sym_block_comment_repeat1] = aux_sym_block_comment_repeat1,
  [aux_sym_line_comment_repeat1] = aux_sym_line_comment_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_BSLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_i] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_3] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_4] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_STAR] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_block_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_STAR_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_SLASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_line_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_line_comment_token2] = {
    .visible = false,
    .named = false,
  },
  [sym_source] = {
    .visible = true,
    .named = true,
  },
  [sym__term] = {
    .visible = false,
    .named = true,
  },
  [sym_application] = {
    .visible = true,
    .named = true,
  },
  [sym_abstraction] = {
    .visible = true,
    .named = true,
  },
  [sym__type] = {
    .visible = false,
    .named = true,
  },
  [sym_base_type] = {
    .visible = true,
    .named = true,
  },
  [sym_function_type] = {
    .visible = true,
    .named = true,
  },
  [sym_block_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_line_comment] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_block_comment_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_line_comment_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_bind_type = 1,
  field_body = 2,
  field_bound = 3,
  field_lhs = 4,
  field_rhs = 5,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_bind_type] = "bind_type",
  [field_body] = "body",
  [field_bound] = "bound",
  [field_lhs] = "lhs",
  [field_rhs] = "rhs",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 2},
  [3] = {.index = 4, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_lhs, 0},
    {field_rhs, 1},
  [2] =
    {field_body, 3},
    {field_bound, 1},
  [4] =
    {field_bind_type, 3},
    {field_body, 5},
    {field_bound, 1},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(10);
      if (lookahead == '(') ADVANCE(11);
      if (lookahead == ')') ADVANCE(12);
      if (lookahead == '*') ADVANCE(7);
      if (lookahead == '-') ADVANCE(8);
      if (lookahead == '.') ADVANCE(17);
      if (lookahead == '/') ADVANCE(5);
      if (lookahead == ':') ADVANCE(16);
      if (lookahead == '\\') ADVANCE(14);
      if (lookahead == 'i') ADVANCE(19);
      if (lookahead == 953) ADVANCE(20);
      if (lookahead == 955) ADVANCE(15);
      if (lookahead == 8594) ADVANCE(22);
      if (lookahead == 10230) ADVANCE(23);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(13);
      END_STATE();
    case 1:
      if (lookahead == '\n') SKIP(1)
      if (lookahead == '*') ADVANCE(28);
      if (lookahead == '/') ADVANCE(26);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(27);
      if (lookahead != 0) ADVANCE(25);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(33);
      if (lookahead == '\r') ADVANCE(3);
      if (lookahead == '/') ADVANCE(32);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(31);
      if (lookahead != 0) ADVANCE(31);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(33);
      if (lookahead == '/') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(6)
      END_STATE();
    case 4:
      if (lookahead == '(') ADVANCE(11);
      if (lookahead == '/') ADVANCE(5);
      if (lookahead == 'i') ADVANCE(18);
      if (lookahead == 953) ADVANCE(20);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(4)
      END_STATE();
    case 5:
      if (lookahead == '*') ADVANCE(24);
      if (lookahead == '/') ADVANCE(30);
      END_STATE();
    case 6:
      if (lookahead == '/') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(6)
      END_STATE();
    case 7:
      if (lookahead == '/') ADVANCE(29);
      END_STATE();
    case 8:
      if (lookahead == '>') ADVANCE(21);
      END_STATE();
    case 9:
      if (eof) ADVANCE(10);
      if (lookahead == '(') ADVANCE(11);
      if (lookahead == ')') ADVANCE(12);
      if (lookahead == '/') ADVANCE(5);
      if (lookahead == '\\') ADVANCE(14);
      if (lookahead == 955) ADVANCE(15);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(9)
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(13);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(13);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_BSLASH);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_i);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_i);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(13);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_2);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_3);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_4);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_SLASH_STAR);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      if (lookahead == '*') ADVANCE(24);
      if (lookahead == '/') ADVANCE(30);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      if (lookahead == '*') ADVANCE(28);
      if (lookahead == '/') ADVANCE(26);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(27);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(25);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      if (lookahead == '/') ADVANCE(29);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_STAR_SLASH);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(aux_sym_line_comment_token1);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(aux_sym_line_comment_token1);
      if (lookahead == '*') ADVANCE(24);
      if (lookahead == '/') ADVANCE(30);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(aux_sym_line_comment_token2);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 9},
  [2] = {.lex_state = 9},
  [3] = {.lex_state = 9},
  [4] = {.lex_state = 9},
  [5] = {.lex_state = 9},
  [6] = {.lex_state = 9},
  [7] = {.lex_state = 9},
  [8] = {.lex_state = 9},
  [9] = {.lex_state = 9},
  [10] = {.lex_state = 4},
  [11] = {.lex_state = 9},
  [12] = {.lex_state = 9},
  [13] = {.lex_state = 4},
  [14] = {.lex_state = 4},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 1},
  [22] = {.lex_state = 1},
  [23] = {.lex_state = 2},
  [24] = {.lex_state = 1},
  [25] = {.lex_state = 2},
  [26] = {.lex_state = 2},
  [27] = {.lex_state = 1},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 2},
  [30] = {.lex_state = 9},
  [31] = {.lex_state = 0},
  [32] = {(TSStateId)(-1)},
  [33] = {(TSStateId)(-1)},
  [34] = {(TSStateId)(-1)},
  [35] = {(TSStateId)(-1)},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [sym_block_comment] = STATE(0),
    [sym_line_comment] = STATE(0),
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_BSLASH] = ACTIONS(1),
    [anon_sym_] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_i] = ACTIONS(1),
    [anon_sym_2] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_3] = ACTIONS(1),
    [anon_sym_4] = ACTIONS(1),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_STAR_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [1] = {
    [sym_source] = STATE(31),
    [sym__term] = STATE(6),
    [sym_application] = STATE(11),
    [sym_abstraction] = STATE(11),
    [sym_block_comment] = STATE(1),
    [sym_line_comment] = STATE(1),
    [anon_sym_LPAREN] = ACTIONS(7),
    [sym_identifier] = ACTIONS(9),
    [anon_sym_BSLASH] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_identifier,
    STATE(3), 1,
      sym__term,
    ACTIONS(11), 2,
      anon_sym_BSLASH,
      anon_sym_,
    ACTIONS(13), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    STATE(2), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(11), 2,
      sym_application,
      sym_abstraction,
  [32] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    STATE(11), 2,
      sym_application,
      sym_abstraction,
    STATE(3), 3,
      sym__term,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(15), 6,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_identifier,
      anon_sym_BSLASH,
      anon_sym_,
  [56] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_identifier,
    STATE(3), 1,
      sym__term,
    ACTIONS(11), 2,
      anon_sym_BSLASH,
      anon_sym_,
    ACTIONS(17), 2,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
    STATE(4), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(11), 2,
      sym_application,
      sym_abstraction,
  [88] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_RPAREN,
    STATE(3), 1,
      sym__term,
    ACTIONS(11), 2,
      anon_sym_BSLASH,
      anon_sym_,
    STATE(5), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(11), 2,
      sym_application,
      sym_abstraction,
  [119] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(21), 1,
      ts_builtin_sym_end,
    STATE(3), 1,
      sym__term,
    ACTIONS(11), 2,
      anon_sym_BSLASH,
      anon_sym_,
    STATE(6), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(11), 2,
      sym_application,
      sym_abstraction,
  [150] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_identifier,
    STATE(4), 1,
      sym__term,
    ACTIONS(11), 2,
      anon_sym_BSLASH,
      anon_sym_,
    STATE(7), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(11), 2,
      sym_application,
      sym_abstraction,
  [178] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_identifier,
    STATE(2), 1,
      sym__term,
    ACTIONS(11), 2,
      anon_sym_BSLASH,
      anon_sym_,
    STATE(8), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(11), 2,
      sym_application,
      sym_abstraction,
  [206] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_identifier,
    STATE(5), 1,
      sym__term,
    ACTIONS(11), 2,
      anon_sym_BSLASH,
      anon_sym_,
    STATE(9), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(11), 2,
      sym_application,
      sym_abstraction,
  [234] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    STATE(16), 1,
      sym__type,
    ACTIONS(25), 2,
      anon_sym_i,
      anon_sym_2,
    STATE(10), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(17), 2,
      sym_base_type,
      sym_function_type,
  [259] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    STATE(11), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(27), 6,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_identifier,
      anon_sym_BSLASH,
      anon_sym_,
  [278] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    STATE(12), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(29), 6,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_identifier,
      anon_sym_BSLASH,
      anon_sym_,
  [297] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    STATE(19), 1,
      sym__type,
    ACTIONS(25), 2,
      anon_sym_i,
      anon_sym_2,
    STATE(13), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(17), 2,
      sym_base_type,
      sym_function_type,
  [322] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(23), 1,
      anon_sym_LPAREN,
    STATE(20), 1,
      sym__type,
    ACTIONS(25), 2,
      anon_sym_i,
      anon_sym_2,
    STATE(14), 2,
      sym_block_comment,
      sym_line_comment,
    STATE(17), 2,
      sym_base_type,
      sym_function_type,
  [347] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    STATE(15), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(31), 5,
      anon_sym_RPAREN,
      anon_sym_DOT,
      anon_sym_DASH_GT,
      anon_sym_3,
      anon_sym_4,
  [365] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(33), 2,
      anon_sym_RPAREN,
      anon_sym_DOT,
    STATE(16), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(35), 3,
      anon_sym_DASH_GT,
      anon_sym_3,
      anon_sym_4,
  [385] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    STATE(17), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(37), 5,
      anon_sym_RPAREN,
      anon_sym_DOT,
      anon_sym_DASH_GT,
      anon_sym_3,
      anon_sym_4,
  [403] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    STATE(18), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(39), 5,
      anon_sym_RPAREN,
      anon_sym_DOT,
      anon_sym_DASH_GT,
      anon_sym_3,
      anon_sym_4,
  [421] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(41), 1,
      anon_sym_DOT,
    STATE(19), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(35), 3,
      anon_sym_DASH_GT,
      anon_sym_3,
      anon_sym_4,
  [440] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(43), 1,
      anon_sym_RPAREN,
    STATE(20), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(35), 3,
      anon_sym_DASH_GT,
      anon_sym_3,
      anon_sym_4,
  [459] = 6,
    ACTIONS(45), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(47), 1,
      aux_sym_block_comment_token1,
    ACTIONS(49), 1,
      anon_sym_STAR_SLASH,
    ACTIONS(51), 1,
      anon_sym_SLASH_SLASH,
    STATE(24), 1,
      aux_sym_block_comment_repeat1,
    STATE(21), 2,
      sym_block_comment,
      sym_line_comment,
  [479] = 6,
    ACTIONS(45), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(47), 1,
      aux_sym_block_comment_token1,
    ACTIONS(51), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(53), 1,
      anon_sym_STAR_SLASH,
    STATE(21), 1,
      aux_sym_block_comment_repeat1,
    STATE(22), 2,
      sym_block_comment,
      sym_line_comment,
  [499] = 6,
    ACTIONS(45), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(51), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(55), 1,
      aux_sym_line_comment_token1,
    ACTIONS(57), 1,
      aux_sym_line_comment_token2,
    STATE(25), 1,
      aux_sym_line_comment_repeat1,
    STATE(23), 2,
      sym_block_comment,
      sym_line_comment,
  [519] = 5,
    ACTIONS(45), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(51), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(59), 1,
      aux_sym_block_comment_token1,
    ACTIONS(62), 1,
      anon_sym_STAR_SLASH,
    STATE(24), 3,
      sym_block_comment,
      sym_line_comment,
      aux_sym_block_comment_repeat1,
  [537] = 5,
    ACTIONS(45), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(51), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(64), 1,
      aux_sym_line_comment_token1,
    ACTIONS(67), 1,
      aux_sym_line_comment_token2,
    STATE(25), 3,
      sym_block_comment,
      sym_line_comment,
      aux_sym_line_comment_repeat1,
  [555] = 6,
    ACTIONS(45), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(51), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(55), 1,
      aux_sym_line_comment_token1,
    ACTIONS(69), 1,
      aux_sym_line_comment_token2,
    STATE(23), 1,
      aux_sym_line_comment_repeat1,
    STATE(26), 2,
      sym_block_comment,
      sym_line_comment,
  [575] = 4,
    ACTIONS(45), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(51), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(71), 2,
      aux_sym_block_comment_token1,
      anon_sym_STAR_SLASH,
    STATE(27), 2,
      sym_block_comment,
      sym_line_comment,
  [590] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(73), 1,
      anon_sym_COLON,
    ACTIONS(75), 1,
      anon_sym_DOT,
    STATE(28), 2,
      sym_block_comment,
      sym_line_comment,
  [607] = 5,
    ACTIONS(45), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(51), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(77), 1,
      aux_sym_line_comment_token1,
    ACTIONS(79), 1,
      aux_sym_line_comment_token2,
    STATE(29), 2,
      sym_block_comment,
      sym_line_comment,
  [624] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(81), 1,
      sym_identifier,
    STATE(30), 2,
      sym_block_comment,
      sym_line_comment,
  [638] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(83), 1,
      ts_builtin_sym_end,
    STATE(31), 2,
      sym_block_comment,
      sym_line_comment,
  [652] = 1,
    ACTIONS(85), 1,
      ts_builtin_sym_end,
  [656] = 1,
    ACTIONS(87), 1,
      ts_builtin_sym_end,
  [660] = 1,
    ACTIONS(89), 1,
      ts_builtin_sym_end,
  [664] = 1,
    ACTIONS(91), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 32,
  [SMALL_STATE(4)] = 56,
  [SMALL_STATE(5)] = 88,
  [SMALL_STATE(6)] = 119,
  [SMALL_STATE(7)] = 150,
  [SMALL_STATE(8)] = 178,
  [SMALL_STATE(9)] = 206,
  [SMALL_STATE(10)] = 234,
  [SMALL_STATE(11)] = 259,
  [SMALL_STATE(12)] = 278,
  [SMALL_STATE(13)] = 297,
  [SMALL_STATE(14)] = 322,
  [SMALL_STATE(15)] = 347,
  [SMALL_STATE(16)] = 365,
  [SMALL_STATE(17)] = 385,
  [SMALL_STATE(18)] = 403,
  [SMALL_STATE(19)] = 421,
  [SMALL_STATE(20)] = 440,
  [SMALL_STATE(21)] = 459,
  [SMALL_STATE(22)] = 479,
  [SMALL_STATE(23)] = 499,
  [SMALL_STATE(24)] = 519,
  [SMALL_STATE(25)] = 537,
  [SMALL_STATE(26)] = 555,
  [SMALL_STATE(27)] = 575,
  [SMALL_STATE(28)] = 590,
  [SMALL_STATE(29)] = 607,
  [SMALL_STATE(30)] = 624,
  [SMALL_STATE(31)] = 638,
  [SMALL_STATE(32)] = 652,
  [SMALL_STATE(33)] = 656,
  [SMALL_STATE(34)] = 660,
  [SMALL_STATE(35)] = 664,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_abstraction, 4, .production_id = 2),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_application, 2, .production_id = 1),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_abstraction, 6, .production_id = 3),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 1),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__term, 1),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__term, 3),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__type, 3),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_type, 3),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__type, 1),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_base_type, 1),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [45] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [47] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [49] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [51] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [53] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [55] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [59] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_comment_repeat1, 2), SHIFT_REPEAT(27),
  [62] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_comment_repeat1, 2),
  [64] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_line_comment_repeat1, 2), SHIFT_REPEAT(29),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_line_comment_repeat1, 2),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [71] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_comment_repeat1, 1),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [77] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_line_comment_repeat1, 1),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_line_comment_repeat1, 1),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [83] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_comment, 3),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block_comment, 3),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_comment, 2),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block_comment, 2),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_lambda(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
