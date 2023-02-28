#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 44
#define LARGE_STATE_COUNT 29
#define SYMBOL_COUNT 40
#define ALIAS_COUNT 0
#define TOKEN_COUNT 29
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 8
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 7

enum {
  anon_sym_LPAREN = 1,
  anon_sym_RPAREN = 2,
  anon_sym_LBRACK = 3,
  anon_sym_RBRACK = 4,
  anon_sym_STAR = 5,
  anon_sym_ = 6,
  sym_variable = 7,
  anon_sym_i = 8,
  anon_sym_2 = 9,
  anon_sym_BSLASH = 10,
  anon_sym_3 = 11,
  anon_sym_4 = 12,
  anon_sym_COLON = 13,
  anon_sym_COLON_COLON = 14,
  anon_sym_DOT = 15,
  anon_sym_TT = 16,
  anon_sym_5 = 17,
  anon_sym_6 = 18,
  aux_sym_function_token1 = 19,
  anon_sym_DASH_GT = 20,
  anon_sym_7 = 21,
  anon_sym_8 = 22,
  anon_sym_SLASH_STAR = 23,
  aux_sym_block_comment_token1 = 24,
  anon_sym_STAR_SLASH = 25,
  anon_sym_SLASH_SLASH = 26,
  aux_sym_line_comment_token1 = 27,
  aux_sym_line_comment_token2 = 28,
  sym_source = 29,
  sym__term = 30,
  sym_sort = 31,
  sym_constant = 32,
  sym_application = 33,
  sym_abstraction = 34,
  sym_function = 35,
  sym_block_comment = 36,
  sym_line_comment = 37,
  aux_sym_block_comment_repeat1 = 38,
  aux_sym_line_comment_repeat1 = 39,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_STAR] = "*",
  [anon_sym_] = "◻",
  [sym_variable] = "variable",
  [anon_sym_i] = "i",
  [anon_sym_2] = "ι",
  [anon_sym_BSLASH] = "\\",
  [anon_sym_3] = "λ",
  [anon_sym_4] = "Λ",
  [anon_sym_COLON] = ":",
  [anon_sym_COLON_COLON] = "::",
  [anon_sym_DOT] = ".",
  [anon_sym_TT] = "TT",
  [anon_sym_5] = "Π",
  [anon_sym_6] = "∀",
  [aux_sym_function_token1] = "function_token1",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_7] = "→",
  [anon_sym_8] = "⟶",
  [anon_sym_SLASH_STAR] = "/*",
  [aux_sym_block_comment_token1] = "block_comment_token1",
  [anon_sym_STAR_SLASH] = "*/",
  [anon_sym_SLASH_SLASH] = "//",
  [aux_sym_line_comment_token1] = "line_comment_token1",
  [aux_sym_line_comment_token2] = "line_comment_token2",
  [sym_source] = "source",
  [sym__term] = "_term",
  [sym_sort] = "sort",
  [sym_constant] = "constant",
  [sym_application] = "application",
  [sym_abstraction] = "abstraction",
  [sym_function] = "function",
  [sym_block_comment] = "block_comment",
  [sym_line_comment] = "line_comment",
  [aux_sym_block_comment_repeat1] = "block_comment_repeat1",
  [aux_sym_line_comment_repeat1] = "line_comment_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_] = anon_sym_,
  [sym_variable] = sym_variable,
  [anon_sym_i] = anon_sym_i,
  [anon_sym_2] = anon_sym_2,
  [anon_sym_BSLASH] = anon_sym_BSLASH,
  [anon_sym_3] = anon_sym_3,
  [anon_sym_4] = anon_sym_4,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_COLON_COLON] = anon_sym_COLON_COLON,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_TT] = anon_sym_TT,
  [anon_sym_5] = anon_sym_5,
  [anon_sym_6] = anon_sym_6,
  [aux_sym_function_token1] = aux_sym_function_token1,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_7] = anon_sym_7,
  [anon_sym_8] = anon_sym_8,
  [anon_sym_SLASH_STAR] = anon_sym_SLASH_STAR,
  [aux_sym_block_comment_token1] = aux_sym_block_comment_token1,
  [anon_sym_STAR_SLASH] = anon_sym_STAR_SLASH,
  [anon_sym_SLASH_SLASH] = anon_sym_SLASH_SLASH,
  [aux_sym_line_comment_token1] = aux_sym_line_comment_token1,
  [aux_sym_line_comment_token2] = aux_sym_line_comment_token2,
  [sym_source] = sym_source,
  [sym__term] = sym__term,
  [sym_sort] = sym_sort,
  [sym_constant] = sym_constant,
  [sym_application] = sym_application,
  [sym_abstraction] = sym_abstraction,
  [sym_function] = sym_function,
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
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_] = {
    .visible = true,
    .named = false,
  },
  [sym_variable] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_i] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BSLASH] = {
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
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_5] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_6] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_function_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_7] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_8] = {
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
  [sym_sort] = {
    .visible = true,
    .named = true,
  },
  [sym_constant] = {
    .visible = true,
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
  [sym_function] = {
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
  field_input = 4,
  field_input_name = 5,
  field_lhs = 6,
  field_output = 7,
  field_rhs = 8,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_bind_type] = "bind_type",
  [field_body] = "body",
  [field_bound] = "bound",
  [field_input] = "input",
  [field_input_name] = "input_name",
  [field_lhs] = "lhs",
  [field_output] = "output",
  [field_rhs] = "rhs",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 2},
  [3] = {.index = 4, .length = 2},
  [4] = {.index = 6, .length = 2},
  [5] = {.index = 8, .length = 3},
  [6] = {.index = 11, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_lhs, 0},
    {field_rhs, 1},
  [2] =
    {field_input, 0},
    {field_output, 2},
  [4] =
    {field_body, 3},
    {field_bound, 1},
  [6] =
    {field_input, 1},
    {field_output, 3},
  [8] =
    {field_bind_type, 3},
    {field_body, 5},
    {field_bound, 1},
  [11] =
    {field_input, 3},
    {field_input_name, 1},
    {field_output, 5},
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
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(9);
      if (lookahead == '(') ADVANCE(10);
      if (lookahead == ')') ADVANCE(11);
      if (lookahead == '*') ADVANCE(15);
      if (lookahead == '-') ADVANCE(7);
      if (lookahead == '.') ADVANCE(27);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == ':') ADVANCE(25);
      if (lookahead == 'T') ADVANCE(18);
      if (lookahead == '[') ADVANCE(12);
      if (lookahead == '\\') ADVANCE(22);
      if (lookahead == ']') ADVANCE(13);
      if (lookahead == 'i') ADVANCE(20);
      if (lookahead == 923) ADVANCE(24);
      if (lookahead == 928) ADVANCE(29);
      if (lookahead == 953) ADVANCE(21);
      if (lookahead == 955) ADVANCE(23);
      if (lookahead == 8594) ADVANCE(34);
      if (lookahead == 8704) ADVANCE(30);
      if (lookahead == 9723) ADVANCE(16);
      if (lookahead == 10230) ADVANCE(35);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == '\r') ADVANCE(2);
      if (lookahead == '/') ADVANCE(44);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(43);
      if (lookahead != 0) ADVANCE(43);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(45);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(6)
      END_STATE();
    case 3:
      if (lookahead == '\n') SKIP(3)
      if (lookahead == '*') ADVANCE(40);
      if (lookahead == '/') ADVANCE(38);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(39);
      if (lookahead != 0) ADVANCE(37);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(36);
      if (lookahead == '/') ADVANCE(42);
      END_STATE();
    case 5:
      if (lookahead == '.') ADVANCE(27);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == ':') ADVANCE(25);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(5)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 6:
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(6)
      END_STATE();
    case 7:
      if (lookahead == '>') ADVANCE(33);
      END_STATE();
    case 8:
      if (eof) ADVANCE(9);
      if (lookahead == '(') ADVANCE(10);
      if (lookahead == ')') ADVANCE(11);
      if (lookahead == '*') ADVANCE(14);
      if (lookahead == '-') ADVANCE(7);
      if (lookahead == '.') ADVANCE(27);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == ':') ADVANCE(32);
      if (lookahead == 'T') ADVANCE(18);
      if (lookahead == '[') ADVANCE(12);
      if (lookahead == '\\') ADVANCE(22);
      if (lookahead == ']') ADVANCE(13);
      if (lookahead == 'i') ADVANCE(20);
      if (lookahead == 923) ADVANCE(24);
      if (lookahead == 928) ADVANCE(29);
      if (lookahead == 953) ADVANCE(21);
      if (lookahead == 955) ADVANCE(23);
      if (lookahead == 8594) ADVANCE(34);
      if (lookahead == 8704) ADVANCE(30);
      if (lookahead == 9723) ADVANCE(16);
      if (lookahead == 10230) ADVANCE(35);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(8)
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_STAR);
      if (lookahead == '/') ADVANCE(41);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_variable);
      if (lookahead == '\'') ADVANCE(17);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_variable);
      if (lookahead == '\'') ADVANCE(17);
      if (lookahead == 'T') ADVANCE(28);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym_variable);
      if (lookahead == '\'') ADVANCE(17);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_i);
      if (lookahead == '\'') ADVANCE(17);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_2);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_BSLASH);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_3);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_4);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == ':') ADVANCE(26);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_COLON_COLON);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_TT);
      if (lookahead == '\'') ADVANCE(17);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(19);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_5);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_6);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(aux_sym_function_token1);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(aux_sym_function_token1);
      if (lookahead == ':') ADVANCE(31);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_7);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_8);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_SLASH_STAR);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      if (lookahead == '*') ADVANCE(36);
      if (lookahead == '/') ADVANCE(42);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      if (lookahead == '*') ADVANCE(40);
      if (lookahead == '/') ADVANCE(38);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(39);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(37);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      if (lookahead == '/') ADVANCE(41);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_STAR_SLASH);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(aux_sym_line_comment_token1);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(aux_sym_line_comment_token1);
      if (lookahead == '*') ADVANCE(36);
      if (lookahead == '/') ADVANCE(42);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(aux_sym_line_comment_token2);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 8},
  [2] = {.lex_state = 8},
  [3] = {.lex_state = 8},
  [4] = {.lex_state = 8},
  [5] = {.lex_state = 8},
  [6] = {.lex_state = 8},
  [7] = {.lex_state = 8},
  [8] = {.lex_state = 8},
  [9] = {.lex_state = 8},
  [10] = {.lex_state = 8},
  [11] = {.lex_state = 8},
  [12] = {.lex_state = 8},
  [13] = {.lex_state = 8},
  [14] = {.lex_state = 8},
  [15] = {.lex_state = 8},
  [16] = {.lex_state = 8},
  [17] = {.lex_state = 8},
  [18] = {.lex_state = 8},
  [19] = {.lex_state = 8},
  [20] = {.lex_state = 8},
  [21] = {.lex_state = 8},
  [22] = {.lex_state = 8},
  [23] = {.lex_state = 8},
  [24] = {.lex_state = 8},
  [25] = {.lex_state = 8},
  [26] = {.lex_state = 8},
  [27] = {.lex_state = 8},
  [28] = {.lex_state = 8},
  [29] = {.lex_state = 1},
  [30] = {.lex_state = 3},
  [31] = {.lex_state = 3},
  [32] = {.lex_state = 5},
  [33] = {.lex_state = 1},
  [34] = {.lex_state = 3},
  [35] = {.lex_state = 1},
  [36] = {.lex_state = 1},
  [37] = {.lex_state = 3},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 5},
  [40] = {(TSStateId)(-1)},
  [41] = {(TSStateId)(-1)},
  [42] = {(TSStateId)(-1)},
  [43] = {(TSStateId)(-1)},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [sym_block_comment] = STATE(0),
    [sym_line_comment] = STATE(0),
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_] = ACTIONS(1),
    [sym_variable] = ACTIONS(1),
    [anon_sym_i] = ACTIONS(1),
    [anon_sym_2] = ACTIONS(1),
    [anon_sym_BSLASH] = ACTIONS(1),
    [anon_sym_3] = ACTIONS(1),
    [anon_sym_4] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_COLON_COLON] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_TT] = ACTIONS(1),
    [anon_sym_5] = ACTIONS(1),
    [anon_sym_6] = ACTIONS(1),
    [aux_sym_function_token1] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_7] = ACTIONS(1),
    [anon_sym_8] = ACTIONS(1),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_STAR_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [1] = {
    [sym_source] = STATE(38),
    [sym__term] = STATE(13),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(1),
    [sym_line_comment] = STATE(1),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [2] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(2),
    [sym_line_comment] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(25),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(25),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_RBRACK] = ACTIONS(25),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_DOT] = ACTIONS(25),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [3] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(3),
    [sym_line_comment] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(29),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(29),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_RBRACK] = ACTIONS(29),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_DOT] = ACTIONS(29),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [4] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(4),
    [sym_line_comment] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(31),
    [anon_sym_LPAREN] = ACTIONS(31),
    [anon_sym_RPAREN] = ACTIONS(31),
    [anon_sym_LBRACK] = ACTIONS(31),
    [anon_sym_RBRACK] = ACTIONS(31),
    [anon_sym_STAR] = ACTIONS(31),
    [anon_sym_] = ACTIONS(31),
    [sym_variable] = ACTIONS(33),
    [anon_sym_i] = ACTIONS(33),
    [anon_sym_2] = ACTIONS(31),
    [anon_sym_BSLASH] = ACTIONS(31),
    [anon_sym_3] = ACTIONS(31),
    [anon_sym_4] = ACTIONS(31),
    [anon_sym_DOT] = ACTIONS(31),
    [anon_sym_TT] = ACTIONS(33),
    [anon_sym_5] = ACTIONS(31),
    [anon_sym_6] = ACTIONS(31),
    [anon_sym_DASH_GT] = ACTIONS(31),
    [anon_sym_7] = ACTIONS(31),
    [anon_sym_8] = ACTIONS(31),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [5] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(5),
    [sym_line_comment] = STATE(5),
    [ts_builtin_sym_end] = ACTIONS(35),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(35),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_RBRACK] = ACTIONS(35),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_DOT] = ACTIONS(35),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [6] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(6),
    [sym_line_comment] = STATE(6),
    [ts_builtin_sym_end] = ACTIONS(37),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(37),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_RBRACK] = ACTIONS(37),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_DOT] = ACTIONS(37),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [7] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(7),
    [sym_line_comment] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(39),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(39),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_RBRACK] = ACTIONS(39),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_DOT] = ACTIONS(39),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [8] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(8),
    [sym_line_comment] = STATE(8),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_DOT] = ACTIONS(41),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [9] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(9),
    [sym_line_comment] = STATE(9),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_DOT] = ACTIONS(43),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [10] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(10),
    [sym_line_comment] = STATE(10),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_RBRACK] = ACTIONS(45),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [11] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(11),
    [sym_line_comment] = STATE(11),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(45),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [12] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(12),
    [sym_line_comment] = STATE(12),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_DOT] = ACTIONS(47),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [13] = {
    [sym__term] = STATE(4),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(13),
    [sym_line_comment] = STATE(13),
    [ts_builtin_sym_end] = ACTIONS(49),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_DASH_GT] = ACTIONS(27),
    [anon_sym_7] = ACTIONS(27),
    [anon_sym_8] = ACTIONS(27),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [14] = {
    [sym_block_comment] = STATE(14),
    [sym_line_comment] = STATE(14),
    [ts_builtin_sym_end] = ACTIONS(51),
    [anon_sym_LPAREN] = ACTIONS(51),
    [anon_sym_RPAREN] = ACTIONS(51),
    [anon_sym_LBRACK] = ACTIONS(51),
    [anon_sym_RBRACK] = ACTIONS(51),
    [anon_sym_STAR] = ACTIONS(51),
    [anon_sym_] = ACTIONS(51),
    [sym_variable] = ACTIONS(53),
    [anon_sym_i] = ACTIONS(53),
    [anon_sym_2] = ACTIONS(51),
    [anon_sym_BSLASH] = ACTIONS(51),
    [anon_sym_3] = ACTIONS(51),
    [anon_sym_4] = ACTIONS(51),
    [anon_sym_DOT] = ACTIONS(51),
    [anon_sym_TT] = ACTIONS(53),
    [anon_sym_5] = ACTIONS(51),
    [anon_sym_6] = ACTIONS(51),
    [anon_sym_DASH_GT] = ACTIONS(51),
    [anon_sym_7] = ACTIONS(51),
    [anon_sym_8] = ACTIONS(51),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [15] = {
    [sym_block_comment] = STATE(15),
    [sym_line_comment] = STATE(15),
    [ts_builtin_sym_end] = ACTIONS(55),
    [anon_sym_LPAREN] = ACTIONS(55),
    [anon_sym_RPAREN] = ACTIONS(55),
    [anon_sym_LBRACK] = ACTIONS(55),
    [anon_sym_RBRACK] = ACTIONS(55),
    [anon_sym_STAR] = ACTIONS(55),
    [anon_sym_] = ACTIONS(55),
    [sym_variable] = ACTIONS(57),
    [anon_sym_i] = ACTIONS(57),
    [anon_sym_2] = ACTIONS(55),
    [anon_sym_BSLASH] = ACTIONS(55),
    [anon_sym_3] = ACTIONS(55),
    [anon_sym_4] = ACTIONS(55),
    [anon_sym_DOT] = ACTIONS(55),
    [anon_sym_TT] = ACTIONS(57),
    [anon_sym_5] = ACTIONS(55),
    [anon_sym_6] = ACTIONS(55),
    [anon_sym_DASH_GT] = ACTIONS(55),
    [anon_sym_7] = ACTIONS(55),
    [anon_sym_8] = ACTIONS(55),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [16] = {
    [sym_block_comment] = STATE(16),
    [sym_line_comment] = STATE(16),
    [ts_builtin_sym_end] = ACTIONS(59),
    [anon_sym_LPAREN] = ACTIONS(59),
    [anon_sym_RPAREN] = ACTIONS(59),
    [anon_sym_LBRACK] = ACTIONS(59),
    [anon_sym_RBRACK] = ACTIONS(59),
    [anon_sym_STAR] = ACTIONS(59),
    [anon_sym_] = ACTIONS(59),
    [sym_variable] = ACTIONS(61),
    [anon_sym_i] = ACTIONS(61),
    [anon_sym_2] = ACTIONS(59),
    [anon_sym_BSLASH] = ACTIONS(59),
    [anon_sym_3] = ACTIONS(59),
    [anon_sym_4] = ACTIONS(59),
    [anon_sym_DOT] = ACTIONS(59),
    [anon_sym_TT] = ACTIONS(61),
    [anon_sym_5] = ACTIONS(59),
    [anon_sym_6] = ACTIONS(59),
    [anon_sym_DASH_GT] = ACTIONS(59),
    [anon_sym_7] = ACTIONS(59),
    [anon_sym_8] = ACTIONS(59),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [17] = {
    [sym_block_comment] = STATE(17),
    [sym_line_comment] = STATE(17),
    [ts_builtin_sym_end] = ACTIONS(63),
    [anon_sym_LPAREN] = ACTIONS(63),
    [anon_sym_RPAREN] = ACTIONS(63),
    [anon_sym_LBRACK] = ACTIONS(63),
    [anon_sym_RBRACK] = ACTIONS(63),
    [anon_sym_STAR] = ACTIONS(63),
    [anon_sym_] = ACTIONS(63),
    [sym_variable] = ACTIONS(65),
    [anon_sym_i] = ACTIONS(65),
    [anon_sym_2] = ACTIONS(63),
    [anon_sym_BSLASH] = ACTIONS(63),
    [anon_sym_3] = ACTIONS(63),
    [anon_sym_4] = ACTIONS(63),
    [anon_sym_DOT] = ACTIONS(63),
    [anon_sym_TT] = ACTIONS(65),
    [anon_sym_5] = ACTIONS(63),
    [anon_sym_6] = ACTIONS(63),
    [anon_sym_DASH_GT] = ACTIONS(63),
    [anon_sym_7] = ACTIONS(63),
    [anon_sym_8] = ACTIONS(63),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [18] = {
    [sym__term] = STATE(11),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(18),
    [sym_line_comment] = STATE(18),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [19] = {
    [sym__term] = STATE(3),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(19),
    [sym_line_comment] = STATE(19),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [20] = {
    [sym__term] = STATE(9),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(20),
    [sym_line_comment] = STATE(20),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(67),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [21] = {
    [sym__term] = STATE(7),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(21),
    [sym_line_comment] = STATE(21),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [22] = {
    [sym__term] = STATE(2),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(22),
    [sym_line_comment] = STATE(22),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [23] = {
    [sym__term] = STATE(8),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(23),
    [sym_line_comment] = STATE(23),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [24] = {
    [sym__term] = STATE(6),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(24),
    [sym_line_comment] = STATE(24),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [25] = {
    [sym__term] = STATE(5),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(25),
    [sym_line_comment] = STATE(25),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [26] = {
    [sym__term] = STATE(10),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(26),
    [sym_line_comment] = STATE(26),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [27] = {
    [sym__term] = STATE(12),
    [sym_sort] = STATE(14),
    [sym_constant] = STATE(14),
    [sym_application] = STATE(14),
    [sym_abstraction] = STATE(14),
    [sym_function] = STATE(14),
    [sym_block_comment] = STATE(27),
    [sym_line_comment] = STATE(27),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_STAR] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [sym_variable] = ACTIONS(13),
    [anon_sym_i] = ACTIONS(15),
    [anon_sym_2] = ACTIONS(17),
    [anon_sym_BSLASH] = ACTIONS(19),
    [anon_sym_3] = ACTIONS(19),
    [anon_sym_4] = ACTIONS(19),
    [anon_sym_TT] = ACTIONS(21),
    [anon_sym_5] = ACTIONS(23),
    [anon_sym_6] = ACTIONS(23),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [28] = {
    [sym_block_comment] = STATE(28),
    [sym_line_comment] = STATE(28),
    [anon_sym_LPAREN] = ACTIONS(51),
    [anon_sym_LBRACK] = ACTIONS(51),
    [anon_sym_STAR] = ACTIONS(51),
    [anon_sym_] = ACTIONS(51),
    [sym_variable] = ACTIONS(53),
    [anon_sym_i] = ACTIONS(53),
    [anon_sym_2] = ACTIONS(51),
    [anon_sym_BSLASH] = ACTIONS(51),
    [anon_sym_3] = ACTIONS(51),
    [anon_sym_4] = ACTIONS(51),
    [anon_sym_DOT] = ACTIONS(51),
    [anon_sym_TT] = ACTIONS(53),
    [anon_sym_5] = ACTIONS(51),
    [anon_sym_6] = ACTIONS(51),
    [aux_sym_function_token1] = ACTIONS(69),
    [anon_sym_DASH_GT] = ACTIONS(51),
    [anon_sym_7] = ACTIONS(51),
    [anon_sym_8] = ACTIONS(51),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(71), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(73), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(75), 1,
      aux_sym_line_comment_token1,
    ACTIONS(78), 1,
      aux_sym_line_comment_token2,
    STATE(29), 3,
      sym_block_comment,
      sym_line_comment,
      aux_sym_line_comment_repeat1,
  [18] = 5,
    ACTIONS(71), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(73), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(80), 1,
      aux_sym_block_comment_token1,
    ACTIONS(83), 1,
      anon_sym_STAR_SLASH,
    STATE(30), 3,
      sym_block_comment,
      sym_line_comment,
      aux_sym_block_comment_repeat1,
  [36] = 6,
    ACTIONS(71), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(73), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(85), 1,
      aux_sym_block_comment_token1,
    ACTIONS(87), 1,
      anon_sym_STAR_SLASH,
    STATE(34), 1,
      aux_sym_block_comment_repeat1,
    STATE(31), 2,
      sym_block_comment,
      sym_line_comment,
  [56] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(89), 1,
      anon_sym_COLON,
    ACTIONS(91), 1,
      anon_sym_COLON_COLON,
    ACTIONS(93), 1,
      anon_sym_DOT,
    STATE(32), 2,
      sym_block_comment,
      sym_line_comment,
  [76] = 6,
    ACTIONS(71), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(73), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(95), 1,
      aux_sym_line_comment_token1,
    ACTIONS(97), 1,
      aux_sym_line_comment_token2,
    STATE(29), 1,
      aux_sym_line_comment_repeat1,
    STATE(33), 2,
      sym_block_comment,
      sym_line_comment,
  [96] = 6,
    ACTIONS(71), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(73), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(85), 1,
      aux_sym_block_comment_token1,
    ACTIONS(99), 1,
      anon_sym_STAR_SLASH,
    STATE(30), 1,
      aux_sym_block_comment_repeat1,
    STATE(34), 2,
      sym_block_comment,
      sym_line_comment,
  [116] = 6,
    ACTIONS(71), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(73), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(95), 1,
      aux_sym_line_comment_token1,
    ACTIONS(101), 1,
      aux_sym_line_comment_token2,
    STATE(33), 1,
      aux_sym_line_comment_repeat1,
    STATE(35), 2,
      sym_block_comment,
      sym_line_comment,
  [136] = 5,
    ACTIONS(71), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(73), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(103), 1,
      aux_sym_line_comment_token1,
    ACTIONS(105), 1,
      aux_sym_line_comment_token2,
    STATE(36), 2,
      sym_block_comment,
      sym_line_comment,
  [153] = 4,
    ACTIONS(71), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(73), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(107), 2,
      aux_sym_block_comment_token1,
      anon_sym_STAR_SLASH,
    STATE(37), 2,
      sym_block_comment,
      sym_line_comment,
  [168] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(109), 1,
      ts_builtin_sym_end,
    STATE(38), 2,
      sym_block_comment,
      sym_line_comment,
  [182] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(111), 1,
      sym_variable,
    STATE(39), 2,
      sym_block_comment,
      sym_line_comment,
  [196] = 1,
    ACTIONS(113), 1,
      ts_builtin_sym_end,
  [200] = 1,
    ACTIONS(115), 1,
      ts_builtin_sym_end,
  [204] = 1,
    ACTIONS(117), 1,
      ts_builtin_sym_end,
  [208] = 1,
    ACTIONS(119), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(29)] = 0,
  [SMALL_STATE(30)] = 18,
  [SMALL_STATE(31)] = 36,
  [SMALL_STATE(32)] = 56,
  [SMALL_STATE(33)] = 76,
  [SMALL_STATE(34)] = 96,
  [SMALL_STATE(35)] = 116,
  [SMALL_STATE(36)] = 136,
  [SMALL_STATE(37)] = 153,
  [SMALL_STATE(38)] = 168,
  [SMALL_STATE(39)] = 182,
  [SMALL_STATE(40)] = 196,
  [SMALL_STATE(41)] = 200,
  [SMALL_STATE(42)] = 204,
  [SMALL_STATE(43)] = 208,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function, 6, .production_id = 6),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_abstraction, 6, .production_id = 5),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_application, 2, .production_id = 1),
  [33] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_application, 2, .production_id = 1),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function, 3, .production_id = 2),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_abstraction, 4, .production_id = 3),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function, 4, .production_id = 4),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 1),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__term, 1),
  [53] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__term, 1),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constant, 1),
  [57] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constant, 1),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sort, 1),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sort, 1),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__term, 3),
  [65] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__term, 3),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [71] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [75] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_line_comment_repeat1, 2), SHIFT_REPEAT(36),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_line_comment_repeat1, 2),
  [80] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_comment_repeat1, 2), SHIFT_REPEAT(37),
  [83] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_comment_repeat1, 2),
  [85] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [87] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [89] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [93] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [95] = {.entry = {.count = 1, .reusable = false}}, SHIFT(36),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [99] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [103] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_line_comment_repeat1, 1),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_line_comment_repeat1, 1),
  [107] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_comment_repeat1, 1),
  [109] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block_comment, 3),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_comment, 3),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_comment, 2),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block_comment, 2),
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
