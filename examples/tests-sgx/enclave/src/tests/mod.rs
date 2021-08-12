mod uri;

extern crate httparse;
use httparse::{Error, Request, Response, Status, EMPTY_HEADER, parse_chunk_size, ParserConfig, InvalidChunkSize};
use core::slice;

pub fn tests_httparse() {

    print!("Testing iter::tests::test_next_8_extra()...");
    iter::tests::test_next_8_extra();
    print!("Success.\n");

    print!("Testing iter::tests::test_next_8_too_short()...");
    iter::tests::test_next_8_too_short();
    print!("Success.\n");

    print!("Testing iter::tests::test_next_8_just_right()...");
    iter::tests::test_next_8_just_right();
    print!("Success.\n");

    print!("Testing tests::test_allow_response_with_whitespace_between_header_name_and_colon()...");
    tests::test_allow_response_with_whitespace_between_header_name_and_colon();
    print!("Success.\n");

    print!("Testing simd::sse42::sse_code_matches_uri_chars_table()...");
    //simd::sse42::sse_code_matches_uri_chars_table();
    print!("Not tested.\n");

    print!("Testing simd::avx2::avx2_code_matches_uri_chars_table()...");
    //simd::avx2::avx2_code_matches_uri_chars_table();
    print!("Not tested.\n");

    print!("Testing tests::test_chunk_size()...");
    tests::test_chunk_size();
    print!("Success.\n");

    print!("Testing tests::test_forbid_request_with_whitespace_between_header_name_and_colon()...");
    tests::test_forbid_request_with_whitespace_between_header_name_and_colon();
    print!("Success.\n");

    print!("Testing tests::test_request_empty_lines_prefix()...");
    tests::test_request_empty_lines_prefix();
    print!("Success.\n");

    print!("Testing tests::test_request_empty_lines_prefix_lf_only()...");
    tests::test_request_empty_lines_prefix_lf_only();
    print!("Success.\n");

    print!("Testing tests::test_request_header_value_htab_long()...");
    tests::test_request_header_value_htab_long();
    print!("Success.\n");

    print!("Testing tests::test_request_header_value_htab_med()...");
    tests::test_request_header_value_htab_med();
    print!("Success.\n");

    print!("Testing tests::test_request_header_value_htab_short()...");
    tests::test_request_header_value_htab_short();
    print!("Success.\n");

    print!("Testing tests::test_request_headers()...");
    tests::test_request_headers();
    print!("Success.\n");

    print!("Testing tests::test_request_headers_max()...");
    tests::test_request_headers_max();
    print!("Success.\n");

    print!("Testing tests::test_request_multibyte()...");
    tests::test_request_multibyte();
    print!("Success.\n");

    print!("Testing tests::test_request_headers_optional_whitespace()...");
    tests::test_request_headers_optional_whitespace();
    print!("Success.\n");

    print!("Testing tests::test_request_partial_version()...");
    tests::test_request_partial_version();
    print!("Success.\n");

    print!("Testing tests::test_request_partial()...");
    tests::test_request_partial();
    print!("Success.\n");

    print!("Testing tests::test_request_newlines()...");
    tests::test_request_newlines();
    print!("Success.\n");

    print!("Testing tests::test_forbid_response_with_whitespace_between_header_name_and_colon()...");
    tests::test_forbid_response_with_whitespace_between_header_name_and_colon();
    print!("Success.\n");

    print!("Testing tests::test_request_path_backslash()...");
    tests::test_request_path_backslash();
    print!("Success.\n");

    print!("Testing tests::test_request_simple()...");
    tests::test_request_simple();
    print!("Success.\n");

    print!("Testing tests::test_request_simple_with_whatwg_query_params()...");
    tests::test_request_simple_with_whatwg_query_params();
    print!("Success.\n");

    print!("Testing tests::test_request_with_invalid_token_delimiter()...");
    tests::test_request_with_invalid_token_delimiter();
    print!("Success.\n");

    print!("Testing tests::test_request_simple_with_query_params()...");
    tests::test_request_simple_with_query_params();
    print!("Success.\n");

    print!("Testing tests::test_response_code_missing_space()...");
    tests::test_response_code_missing_space();
    print!("Success.\n");

    print!("Testing tests::test_response_empty_lines_prefix_lf_only()...");
    tests::test_response_empty_lines_prefix_lf_only();
    print!("Success.\n");

    print!("Testing tests::test_response_newlines()...");
    tests::test_response_newlines();
    print!("Success.\n");

    print!("Testing tests::test_response_no_cr()...");
    tests::test_response_no_cr();
    print!("Success.\n");

    print!("Testing tests::test_response_reason_missing()...");
    tests::test_response_reason_missing();
    print!("Success.\n");

    print!("Testing tests::test_response_reason_missing_no_space()...");
    tests::test_response_reason_missing_no_space();
    print!("Success.\n");

    print!("Testing tests::test_response_reason_missing_no_space_with_headers()...");
    tests::test_response_reason_missing_no_space_with_headers();
    print!("Success.\n");

    print!("Testing tests::test_response_reason_with_nul_byte()...");
    tests::test_response_reason_with_nul_byte();
    print!("Success.\n");

    print!("Testing tests::test_response_reason_with_obsolete_text_byte()...");
    tests::test_response_reason_with_obsolete_text_byte();
    print!("Success.\n");

    print!("Testing tests::test_response_simple()...");
    tests::test_response_simple();
    print!("Success.\n");

    print!("Testing tests::test_response_reason_with_space_and_tab()...");
    tests::test_response_reason_with_space_and_tab();
    print!("Success.\n");

    print!("Testing tests::test_response_version_missing_space()...");
    tests::test_response_version_missing_space();
    print!("Success.\n");

    print!("Testing tests::test_shrink()...");
    tests::test_shrink();
    print!("Success.\n");

    print!("Testing tests::test_std_error()...");
    tests::test_std_error();
    print!("Success.\n");

    print!("Testing tests::test_request_with_invalid_but_short_version()...");
    tests::test_request_with_invalid_but_short_version();
    print!("Success.\n");
}

pub fn urltests() {
    print!("Testing urltest_001()...");
    uri::urltest_001();
    print!("Success.\n");

    print!("Testing urltest_002()...");
    uri::urltest_002();
    print!("Success.\n");

    print!("Testing urltest_003()...");
    uri::urltest_003();
    print!("Success.\n");

    print!("Testing urltest_004()...");
    uri::urltest_004();
    print!("Success.\n");

    print!("Testing urltest_005()...");
    uri::urltest_005();
    print!("Success.\n");

    print!("Testing urltest_006()...");
    uri::urltest_006();
    print!("Success.\n");

    print!("Testing urltest_007()...");
    uri::urltest_007();
    print!("Success.\n");

    print!("Testing urltest_008()...");
    uri::urltest_008();
    print!("Success.\n");

    print!("Testing urltest_009()...");
    uri::urltest_009();
    print!("Success.\n");

    print!("Testing urltest_010...");
    uri::urltest_010();
    print!("Success.\n");

    print!("Testing urltest_011...");
    uri::urltest_011();
    print!("Success.\n");

    print!("Testing urltest_012...");
    uri::urltest_012();
    print!("Success.\n");

    print!("Testing urltest_013...");
    uri::urltest_013();
    print!("Success.\n");

    print!("Testing urltest_014...");
    uri::urltest_014();
    print!("Success.\n");

    print!("Testing urltest_015...");
    uri::urltest_015();
    print!("Success.\n");

    print!("Testing urltest_016...");
    uri::urltest_016();
    print!("Success.\n");

    print!("Testing urltest_017...");
    uri::urltest_017();
    print!("Success.\n");

    print!("Testing urltest_018...");
    uri::urltest_018();
    print!("Success.\n");

    print!("Testing urltest_019...");
    uri::urltest_019();
    print!("Success.\n");

    print!("Testing urltest_020...");
    uri::urltest_020();
    print!("Success.\n");

    print!("Testing urltest_021...");
    uri::urltest_021();
    print!("Success.\n");

    print!("Testing urltest_022...");
    uri::urltest_022();
    print!("Success.\n");

    print!("Testing urltest_023...");
    uri::urltest_023();
    print!("Success.\n");

    print!("Testing urltest_024...");
    uri::urltest_024();
    print!("Success.\n");

    print!("Testing urltest_025...");
    uri::urltest_025();
    print!("Success.\n");

    print!("Testing urltest_026...");
    uri::urltest_026();
    print!("Success.\n");

    print!("Testing urltest_027...");
    uri::urltest_027();
    print!("Success.\n");

    print!("Testing urltest_028...");
    uri::urltest_028();
    print!("Success.\n");

    print!("Testing urltest_029...");
    uri::urltest_029();
    print!("Success.\n");

    print!("Testing urltest_030...");
    uri::urltest_030();
    print!("Success.\n");

    print!("Testing urltest_031...");
    uri::urltest_031();
    print!("Success.\n");

    print!("Testing urltest_032...");
    uri::urltest_032();
    print!("Success.\n");

    print!("Testing urltest_033...");
    uri::urltest_033();
    print!("Success.\n");

    print!("Testing urltest_034...");
    uri::urltest_034();
    print!("Success.\n");

    print!("Testing urltest_035...");
    uri::urltest_035();
    print!("Success.\n");

    print!("Testing urltest_036...");
    uri::urltest_036();
    print!("Success.\n");

    print!("Testing urltest_037...");
    uri::urltest_037();
    print!("Success.\n");

    print!("Testing urltest_038...");
    uri::urltest_038();
    print!("Success.\n");

    print!("Testing urltest_039...");
    uri::urltest_039();
    print!("Success.\n");

    print!("Testing urltest_040...");
    uri::urltest_040();
    print!("Success.\n");

    print!("Testing urltest_041...");
    uri::urltest_041();
    print!("Success.\n");

    print!("Testing urltest_042...");
    uri::urltest_042();
    print!("Success.\n");

    print!("Testing urltest_043...");
    uri::urltest_043();
    print!("Success.\n");

    print!("Testing urltest_044...");
    uri::urltest_044();
    print!("Success.\n");

    print!("Testing urltest_045...");
    uri::urltest_045();
    print!("Success.\n");

    print!("Testing urltest_046...");
    uri::urltest_046();
    print!("Success.\n");

    print!("Testing urltest_047...");
    uri::urltest_047();
    print!("Success.\n");

    print!("Testing urltest_048...");
    uri::urltest_048();
    print!("Success.\n");

    print!("Testing urltest_049...");
    uri::urltest_049();
    print!("Success.\n");

    print!("Testing urltest_050...");
    uri::urltest_050();
    print!("Success.\n");

    print!("Testing urltest_051...");
    uri::urltest_051();
    print!("Success.\n");

    print!("Testing urltest_052...");
    uri::urltest_052();
    print!("Success.\n");

    print!("Testing urltest_053...");
    uri::urltest_053();
    print!("Success.\n");

    print!("Testing urltest_054...");
    uri::urltest_054();
    print!("Success.\n");

    print!("Testing urltest_055...");
    uri::urltest_055();
    print!("Success.\n");

    print!("Testing urltest_056...");
    uri::urltest_056();
    print!("Success.\n");

    print!("Testing urltest_057...");
    uri::urltest_057();
    print!("Success.\n");

    print!("Testing urltest_058...");
    uri::urltest_058();
    print!("Success.\n");

    print!("Testing urltest_059...");
    uri::urltest_059();
    print!("Success.\n");

    print!("Testing urltest_060...");
    uri::urltest_060();
    print!("Success.\n");

    print!("Testing urltest_061...");
    uri::urltest_061();
    print!("Success.\n");

    print!("Testing urltest_062...");
    uri::urltest_062();
    print!("Success.\n");

    print!("Testing urltest_063...");
    uri::urltest_063();
    print!("Success.\n");

    print!("Testing urltest_064...");
    uri::urltest_064();
    print!("Success.\n");

    print!("Testing urltest_065...");
    uri::urltest_065();
    print!("Success.\n");

    print!("Testing urltest_066...");
    uri::urltest_066();
    print!("Success.\n");

    print!("Testing urltest_067...");
    uri::urltest_067();
    print!("Success.\n");

    print!("Testing urltest_068...");
    uri::urltest_068();
    print!("Success.\n");

    print!("Testing urltest_069...");
    uri::urltest_069();
    print!("Success.\n");

    print!("Testing urltest_070...");
    uri::urltest_070();
    print!("Success.\n");

    print!("Testing urltest_071...");
    uri::urltest_071();
    print!("Success.\n");

    print!("Testing urltest_072...");
    uri::urltest_072();
    print!("Success.\n");

    print!("Testing urltest_073...");
    uri::urltest_073();
    print!("Success.\n");

    print!("Testing urltest_074...");
    uri::urltest_074();
    print!("Success.\n");

    print!("Testing urltest_075...");
    uri::urltest_075();
    print!("Success.\n");

    print!("Testing urltest_076...");
    uri::urltest_076();
    print!("Success.\n");

    print!("Testing urltest_077...");
    uri::urltest_077();
    print!("Success.\n");

    print!("Testing urltest_078...");
    uri::urltest_078();
    print!("Success.\n");

    print!("Testing urltest_079...");
    uri::urltest_079();
    print!("Success.\n");

    print!("Testing urltest_080...");
    uri::urltest_080();
    print!("Success.\n");

    print!("Testing urltest_081...");
    uri::urltest_081();
    print!("Success.\n");

    print!("Testing urltest_082...");
    uri::urltest_082();
    print!("Success.\n");

    print!("Testing urltest_083...");
    uri::urltest_083();
    print!("Success.\n");

    print!("Testing urltest_084...");
    uri::urltest_084();
    print!("Success.\n");

    print!("Testing urltest_085...");
    uri::urltest_085();
    print!("Success.\n");

    print!("Testing urltest_086...");
    uri::urltest_086();
    print!("Success.\n");

    print!("Testing urltest_087...");
    uri::urltest_087();
    print!("Success.\n");

    print!("Testing urltest_088...");
    uri::urltest_088();
    print!("Success.\n");

    print!("Testing urltest_089...");
    uri::urltest_089();
    print!("Success.\n");

    print!("Testing urltest_090...");
    uri::urltest_090();
    print!("Success.\n");

    print!("Testing urltest_091...");
    uri::urltest_091();
    print!("Success.\n");
    print!("Testing urltest_092...");
    uri::urltest_092();
    print!("Success.\n");

    print!("Testing urltest_093...");
    uri::urltest_093();
    print!("Success.\n");

    print!("Testing urltest_094...");
    uri::urltest_094();
    print!("Success.\n");

    print!("Testing urltest_095...");
    uri::urltest_095();
    print!("Success.\n");

    print!("Testing urltest_096...");
    uri::urltest_096();
    print!("Success.\n");

    print!("Testing urltest_097...");
    uri::urltest_097();
    print!("Success.\n");
    print!("Testing urltest_098...");
    uri::urltest_098();
    print!("Success.\n");

    print!("Testing urltest_099...");
    uri::urltest_099();
    print!("Success.\n");

    print!("Testing urltest_100()...");
    uri::urltest_100();
    print!("Success.\n");

    print!("Testing urltest_101()...");
    uri::urltest_101();
    print!("Success.\n");

    print!("Testing urltest_102()...");
    uri::urltest_102();
    print!("Success.\n");

    print!("Testing urltest_103()...");
    uri::urltest_103();
    print!("Success.\n");
    print!("Testing urltest_104()...");
    uri::urltest_104();
    print!("Success.\n");

    print!("Testing urltest_105()...");
    uri::urltest_105();
    print!("Success.\n");

    print!("Testing urltest_106()...");
    uri::urltest_106();
    print!("Success.\n");

    print!("Testing urltest_107()...");
    uri::urltest_107();
    print!("Success.\n");

    print!("Testing urltest_108()...");
    uri::urltest_108();
    print!("Success.\n");

    print!("Testing urltest_109()...");
    uri::urltest_109();
    print!("Success.\n");

    print!("Testing urltest_110()...");
    uri::urltest_110();
    print!("Success.\n");

    print!("Testing urltest_111()...");
    uri::urltest_111();
    print!("Success.\n");

    print!("Testing urltest_112()...");
    uri::urltest_112();
    print!("Success.\n");

    print!("Testing urltest_113()...");
    uri::urltest_113();
    print!("Success.\n");

    print!("Testing urltest_114()...");
    uri::urltest_114();
    print!("Success.\n");

    print!("Testing urltest_115()...");
    uri::urltest_115();
    print!("Success.\n");

    print!("Testing urltest_116()...");
    uri::urltest_116();
    print!("Success.\n");

    print!("Testing urltest_117()...");
    uri::urltest_117();
    print!("Success.\n");

    print!("Testing urltest_118()...");
    uri::urltest_118();
    print!("Success.\n");

    print!("Testing urltest_119()...");
    uri::urltest_119();
    print!("Success.\n");

    print!("Testing urltest_120()...");
    uri::urltest_120();
    print!("Success.\n");

    print!("Testing urltest_121()...");
    uri::urltest_121();
    print!("Success.\n");

    print!("Testing urltest_122()...");
    uri::urltest_122();
    print!("Success.\n");

    print!("Testing urltest_123()...");
    uri::urltest_123();
    print!("Success.\n");

    print!("Testing urltest_124()...");
    uri::urltest_124();
    print!("Success.\n");

    print!("Testing urltest_125()...");
    uri::urltest_125();
    print!("Success.\n");

    print!("Testing urltest_126()...");
    uri::urltest_126();
    print!("Success.\n");

    print!("Testing urltest_127()...");
    uri::urltest_127();
    print!("Success.\n");

    print!("Testing urltest_128()...");
    uri::urltest_128();
    print!("Success.\n");

    print!("Testing urltest_129()...");
    uri::urltest_129();
    print!("Success.\n");

    print!("Testing urltest_130()...");
    uri::urltest_130();
    print!("Success.\n");

    print!("Testing urltest_131()...");
    uri::urltest_131();
    print!("Success.\n");

    print!("Testing urltest_132()...");
    uri::urltest_132();
    print!("Success.\n");

    print!("Testing urltest_133()...");
    uri::urltest_133();
    print!("Success.\n");

    print!("Testing urltest_134()...");
    uri::urltest_134();
    print!("Success.\n");

    print!("Testing urltest_135()...");
    uri::urltest_135();
    print!("Success.\n");

    print!("Testing urltest_136()...");
    uri::urltest_136();
    print!("Success.\n");

    print!("Testing urltest_137()...");
    uri::urltest_137();
    print!("Success.\n");

    print!("Testing urltest_138()...");
    uri::urltest_138();
    print!("Success.\n");

    print!("Testing urltest_139()...");
    uri::urltest_139();
    print!("Success.\n");

    print!("Testing urltest_140()...");
    uri::urltest_140();
    print!("Success.\n");
    print!("Testing urltest_141()...");
    uri::urltest_141();
    print!("Success.\n");

    print!("Testing urltest_142()...");
    uri::urltest_142();
    print!("Success.\n");

    print!("Testing urltest_143()...");
    uri::urltest_143();
    print!("Success.\n");

    print!("Testing urltest_144()...");
    uri::urltest_144();
    print!("Success.\n");

    print!("Testing urltest_145()...");
    uri::urltest_145();
    print!("Success.\n");

    print!("Testing urltest_146()...");
    uri::urltest_146();
    print!("Success.\n");

    print!("Testing urltest_147()...");
    uri::urltest_147();
    print!("Success.\n");

    print!("Testing urltest_148()...");
    uri::urltest_148();
    print!("Success.\n");

    print!("Testing urltest_149()...");
    uri::urltest_149();
    print!("Success.\n");

    print!("Testing urltest_150()...");
    uri::urltest_150();
    print!("Success.\n");

    print!("Testing urltest_151()...");
    uri::urltest_151();
    print!("Success.\n");

    print!("Testing urltest_152()...");
    uri::urltest_152();
    print!("Success.\n");

    print!("Testing urltest_153()...");
    uri::urltest_153();
    print!("Success.\n");

    print!("Testing urltest_154()...");
    uri::urltest_154();
    print!("Success.\n");

    print!("Testing urltest_155()...");
    uri::urltest_155();
    print!("Success.\n");

    print!("Testing urltest_156()...");
    uri::urltest_156();
    print!("Success.\n");

    print!("Testing urltest_157()...");
    uri::urltest_157();
    print!("Success.\n");

    print!("Testing urltest_158()...");
    uri::urltest_158();
    print!("Success.\n");

    print!("Testing urltest_159()...");
    uri::urltest_159();
    print!("Success.\n");

    print!("Testing urltest_160()...");
    uri::urltest_160();
    print!("Success.\n");

    print!("Testing urltest_161()...");
    uri::urltest_161();
    print!("Success.\n");

    print!("Testing urltest_162()...");
    uri::urltest_162();
    print!("Success.\n");

    print!("Testing urltest_163()...");
    uri::urltest_163();
    print!("Success.\n");

    print!("Testing urltest_164()...");
    uri::urltest_164();
    print!("Success.\n");

    print!("Testing urltest_165()...");
    uri::urltest_165();
    print!("Success.\n");

    print!("Testing urltest_166()...");
    uri::urltest_166();
    print!("Success.\n");

    print!("Testing urltest_167()...");
    uri::urltest_167();
    print!("Success.\n");

    print!("Testing urltest_168()...");
    uri::urltest_168();
    print!("Success.\n");

    print!("Testing urltest_169()...");
    uri::urltest_169();
    print!("Success.\n");

    print!("Testing urltest_170()...");
    uri::urltest_170();
    print!("Success.\n");

    print!("Testing urltest_171()...");
    uri::urltest_171();
    print!("Success.\n");

    print!("Testing urltest_172()...");
    uri::urltest_172();
    print!("Success.\n");

    print!("Testing urltest_173()...");
    uri::urltest_173();
    print!("Success.\n");

    print!("Testing urltest_174()...");
    uri::urltest_174();
    print!("Success.\n");

    print!("Testing urltest_175()...");
    uri::urltest_175();
    print!("Success.\n");

    print!("Testing urltest_176()...");
    uri::urltest_176();
    print!("Success.\n");

    print!("Testing urltest_177()...");
    uri::urltest_177();
    print!("Success.\n");

    print!("Testing urltest_178()...");
    uri::urltest_178();
    print!("Success.\n");

    print!("Testing urltest_179()...");
    uri::urltest_179();
    print!("Success.\n");

    print!("Testing urltest_180()...");
    uri::urltest_180();
    print!("Success.\n");

    print!("Testing urltest_181()...");
    uri::urltest_181();
    print!("Success.\n");

    print!("Testing urltest_182()...");
    uri::urltest_182();
    print!("Success.\n");

    print!("Testing urltest_183()...");
    uri::urltest_183();
    print!("Success.\n");

    print!("Testing urltest_184()...");
    uri::urltest_184();
    print!("Success.\n");

    print!("Testing urltest_185()...");
    uri::urltest_185();
    print!("Success.\n");

    print!("Testing urltest_186()...");
    uri::urltest_186();
    print!("Success.\n");

    print!("Testing urltest_187()...");
    uri::urltest_187();
    print!("Success.\n");

    print!("Testing urltest_188()...");
    uri::urltest_188();
    print!("Success.\n");

    print!("Testing urltest_189()...");
    uri::urltest_189();
    print!("Success.\n");

    print!("Testing urltest_190()...");
    uri::urltest_190();
    print!("Success.\n");

    print!("Testing urltest_191()...");
    uri::urltest_191();
    print!("Success.\n");

    print!("Testing urltest_192()...");
    uri::urltest_192();
    print!("Success.\n");

    print!("Testing urltest_193()...");
    uri::urltest_193();
    print!("Success.\n");

    print!("Testing urltest_194()...");
    uri::urltest_194();
    print!("Success.\n");
    print!("Testing urltest_195()...");
    uri::urltest_195();
    print!("Success.\n");

    print!("Testing urltest_196()...");
    uri::urltest_196();
    print!("Success.\n");

    print!("Testing urltest_197()...");
    uri::urltest_197();
    print!("Success.\n");

    print!("Testing urltest_198()...");
    uri::urltest_198();
    print!("Success.\n");

    print!("Testing urltest_199()...");
    uri::urltest_199();
    print!("Success.\n");

    print!("Testing urltest_200()...");
    uri::urltest_200();
    print!("Success.\n");

    print!("Testing urltest_201()...");
    uri::urltest_201();
    print!("Success.\n");

    print!("Testing urltest_202()...");
    uri::urltest_202();
    print!("Success.\n");

    print!("Testing urltest_203()...");
    uri::urltest_203();
    print!("Success.\n");

    print!("Testing urltest_204()...");
    uri::urltest_204();
    print!("Success.\n");

    print!("Testing urltest_205()...");
    uri::urltest_205();
    print!("Success.\n");

    print!("Testing urltest_206()...");
    uri::urltest_206();
    print!("Success.\n");

    print!("Testing urltest_207()...");
    uri::urltest_207();
    print!("Success.\n");

    print!("Testing urltest_208()...");
    uri::urltest_208();
    print!("Success.\n");

    print!("Testing urltest_209()...");
    uri::urltest_209();
    print!("Success.\n");

    print!("Testing urltest_210()...");
    uri::urltest_210();
    print!("Success.\n");

    print!("Testing urltest_211()...");
    uri::urltest_211();
    print!("Success.\n");

    print!("Testing urltest_212()...");
    uri::urltest_212();
    print!("Success.\n");

    print!("Testing urltest_213()...");
    uri::urltest_213();
    print!("Success.\n");

    print!("Testing urltest_214()...");
    uri::urltest_214();
    print!("Success.\n");

    print!("Testing urltest_215()...");
    uri::urltest_215();
    print!("Success.\n");

    print!("Testing urltest_216()...");
    uri::urltest_216();
    print!("Success.\n");

    print!("Testing urltest_217()...");
    uri::urltest_217();
    print!("Success.\n");

    print!("Testing urltest_218()...");
    uri::urltest_218();
    print!("Success.\n");

    print!("Testing urltest_219()...");
    uri::urltest_219();
    print!("Success.\n");

    print!("Testing urltest_220()...");
    uri::urltest_220();
    print!("Success.\n");

    print!("Testing urltest_221()...");
    uri::urltest_221();
    print!("Success.\n");

    print!("Testing urltest_222()...");
    uri::urltest_222();
    print!("Success.\n");

    print!("Testing urltest_223()...");
    uri::urltest_223();
    print!("Success.\n");

    print!("Testing urltest_224()...");
    uri::urltest_224();
    print!("Success.\n");

    print!("Testing urltest_225()...");
    uri::urltest_225();
    print!("Success.\n");

    print!("Testing urltest_226()...");
    uri::urltest_226();
    print!("Success.\n");

    print!("Testing urltest_227()...");
    uri::urltest_227();
    print!("Success.\n");

    print!("Testing urltest_228()...");
    uri::urltest_228();
    print!("Success.\n");

    print!("Testing urltest_229()...");
    uri::urltest_229();
    print!("Success.\n");

    print!("Testing urltest_230()...");
    uri::urltest_230();
    print!("Success.\n");

    print!("Testing urltest_231()...");
    uri::urltest_231();
    print!("Success.\n");

    print!("Testing urltest_232()...");
    uri::urltest_232();
    print!("Success.\n");

    print!("Testing urltest_233()...");
    uri::urltest_233();
    print!("Success.\n");

    print!("Testing urltest_234()...");
    uri::urltest_234();
    print!("Success.\n");

    print!("Testing urltest_235()...");
    uri::urltest_235();
    print!("Success.\n");

    print!("Testing urltest_236()...");
    uri::urltest_236();
    print!("Success.\n");

    print!("Testing urltest_237()...");
    uri::urltest_237();
    print!("Success.\n");

    print!("Testing urltest_238()...");
    uri::urltest_238();
    print!("Success.\n");

    print!("Testing urltest_239()...");
    uri::urltest_239();
    print!("Success.\n");

    print!("Testing urltest_240()...");
    uri::urltest_240();
    print!("Success.\n");

    print!("Testing urltest_241()...");
    uri::urltest_241();
    print!("Success.\n");

    print!("Testing urltest_242()...");
    uri::urltest_242();
    print!("Success.\n");

    print!("Testing urltest_243()...");
    uri::urltest_243();
    print!("Success.\n");

    print!("Testing urltest_244()...");
    uri::urltest_244();
    print!("Success.\n");

    print!("Testing urltest_245()...");
    uri::urltest_245();
    print!("Success.\n");

    print!("Testing urltest_246()...");
    uri::urltest_246();
    print!("Success.\n");

    print!("Testing urltest_247()...");
    uri::urltest_247();
    print!("Success.\n");

    print!("Testing urltest_248()...");
    uri::urltest_248();
    print!("Success.\n");

    print!("Testing urltest_249()...");
    uri::urltest_249();
    print!("Success.\n");

    print!("Testing urltest_250()...");
    uri::urltest_250();
    print!("Success.\n");

    print!("Testing urltest_251()...");
    uri::urltest_251();
    print!("Success.\n");

    print!("Testing urltest_252()...");
    uri::urltest_252();
    print!("Success.\n");

    print!("Testing urltest_253()...");
    uri::urltest_253();
    print!("Success.\n");

    print!("Testing urltest_254()...");
    uri::urltest_254();
    print!("Success.\n");

    print!("Testing urltest_255()...");
    uri::urltest_255();
    print!("Success.\n");

    print!("Testing urltest_256()...");
    uri::urltest_256();
    print!("Success.\n");

    print!("Testing urltest_257()...");
    uri::urltest_257();
    print!("Success.\n");

    print!("Testing urltest_258()...");
    uri::urltest_258();
    print!("Success.\n");

    print!("Testing urltest_259()...");
    uri::urltest_259();
    print!("Success.\n");

    print!("Testing urltest_260()...");
    uri::urltest_260();
    print!("Success.\n");

    print!("Testing urltest_261()...");
    uri::urltest_261();
    print!("Success.\n");

    print!("Testing urltest_262()...");
    uri::urltest_262();
    print!("Success.\n");

    print!("Testing urltest_nvidia()...");
    uri::urltest_nvidia();
    print!("Success.\n");


    

}


fn shrink<T>(slice: &mut &mut [T], len: usize) {
    debug_assert!(slice.len() >= len);
    let ptr = slice.as_mut_ptr();
    *slice = unsafe { slice::from_raw_parts_mut(ptr, len) };
}


mod tests {
    use super::{Error, Request, Response, Status, EMPTY_HEADER, shrink, parse_chunk_size, ParserConfig, InvalidChunkSize};

    const NUM_OF_HEADERS: usize = 4;

    //#[test]
    pub fn test_shrink() {
        let mut arr = [EMPTY_HEADER; 16];
        {
            let slice = &mut &mut arr[..];
            assert_eq!(slice.len(), 16);
            shrink(slice, 4);
            assert_eq!(slice.len(), 4);
        }
        assert_eq!(arr.len(), 16);
    }

    macro_rules! req {
        ($name:ident, $buf:expr, |$arg:ident| $body:expr) => (
            req! {$name, $buf, Ok(Status::Complete($buf.len())), |$arg| $body }
        );
        ($name:ident, $buf:expr, $len:expr, |$arg:ident| $body:expr) => (
        //#[test]
        pub fn $name() {
            let mut headers = [EMPTY_HEADER; NUM_OF_HEADERS];
            let mut req = Request::new(&mut headers[..]);
            let status = req.parse($buf.as_ref());
            assert_eq!(status, $len);
            closure(req);

            fn closure($arg: Request) {
                $body
            }
        }
        )
    }

    req! {
        test_request_simple,
        b"GET / HTTP/1.1\r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 0);
        }
    }

    req! {
        test_request_simple_with_query_params,
        b"GET /thing?data=a HTTP/1.1\r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/thing?data=a");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 0);
        }
    }

    req! {
        test_request_simple_with_whatwg_query_params,
        b"GET /thing?data=a^ HTTP/1.1\r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/thing?data=a^");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 0);
        }
    }

    req! {
        test_request_headers,
        b"GET / HTTP/1.1\r\nHost: foo.com\r\nCookie: \r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 2);
            assert_eq!(req.headers[0].name, "Host");
            assert_eq!(req.headers[0].value, b"foo.com");
            assert_eq!(req.headers[1].name, "Cookie");
            assert_eq!(req.headers[1].value, b"");
        }
    }

    req! {
        test_request_headers_optional_whitespace,
        b"GET / HTTP/1.1\r\nHost: \tfoo.com\t \r\nCookie: \t \r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 2);
            assert_eq!(req.headers[0].name, "Host");
            assert_eq!(req.headers[0].value, b"foo.com");
            assert_eq!(req.headers[1].name, "Cookie");
            assert_eq!(req.headers[1].value, b"");
        }
    }

    req! {
        // test the scalar parsing
        test_request_header_value_htab_short,
        b"GET / HTTP/1.1\r\nUser-Agent: some\tagent\r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 1);
            assert_eq!(req.headers[0].name, "User-Agent");
            assert_eq!(req.headers[0].value, b"some\tagent");
        }
    }

    req! {
        // test the sse42 parsing
        test_request_header_value_htab_med,
        b"GET / HTTP/1.1\r\nUser-Agent: 1234567890some\tagent\r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 1);
            assert_eq!(req.headers[0].name, "User-Agent");
            assert_eq!(req.headers[0].value, b"1234567890some\tagent");
        }
    }

    req! {
        // test the avx2 parsing
        test_request_header_value_htab_long,
        b"GET / HTTP/1.1\r\nUser-Agent: 1234567890some\t1234567890agent1234567890\r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 1);
            assert_eq!(req.headers[0].name, "User-Agent");
            assert_eq!(req.headers[0].value, &b"1234567890some\t1234567890agent1234567890"[..]);
        }
    }

    req! {
        test_request_headers_max,
        b"GET / HTTP/1.1\r\nA: A\r\nB: B\r\nC: C\r\nD: D\r\n\r\n",
        |req| {
            assert_eq!(req.headers.len(), NUM_OF_HEADERS);
        }
    }

    req! {
        test_request_multibyte,
        b"GET / HTTP/1.1\r\nHost: foo.com\r\nUser-Agent: \xe3\x81\xb2\xe3/1.0\r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers[0].name, "Host");
            assert_eq!(req.headers[0].value, b"foo.com");
            assert_eq!(req.headers[1].name, "User-Agent");
            assert_eq!(req.headers[1].value, b"\xe3\x81\xb2\xe3/1.0");
        }
    }


    req! {
        test_request_partial,
        b"GET / HTTP/1.1\r\n\r", Ok(Status::Partial),
        |_req| {}
    }

    req! {
        test_request_partial_version,
        b"GET / HTTP/1.", Ok(Status::Partial),
        |_req| {}
    }

    req! {
        test_request_newlines,
        b"GET / HTTP/1.1\nHost: foo.bar\n\n",
        |_r| {}
    }

    req! {
        test_request_empty_lines_prefix,
        b"\r\n\r\nGET / HTTP/1.1\r\n\r\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 0);
        }
    }

    req! {
        test_request_empty_lines_prefix_lf_only,
        b"\n\nGET / HTTP/1.1\n\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 0);
        }
    }

    req! {
        test_request_path_backslash,
        b"\n\nGET /\\?wayne\\=5 HTTP/1.1\n\n",
        |req| {
            assert_eq!(req.method.unwrap(), "GET");
            assert_eq!(req.path.unwrap(), "/\\?wayne\\=5");
            assert_eq!(req.version.unwrap(), 1);
            assert_eq!(req.headers.len(), 0);
        }
    }

    req! {
        test_request_with_invalid_token_delimiter,
        b"GET\n/ HTTP/1.1\r\nHost: foo.bar\r\n\r\n",
        Err(Error::Token),
        |_r| {}
    }


    req! {
        test_request_with_invalid_but_short_version,
        b"GET / HTTP/1!",
        Err(Error::Version),
        |_r| {}
    }

    macro_rules! res {
        ($name:ident, $buf:expr, |$arg:ident| $body:expr) => (
            res! {$name, $buf, Ok(Status::Complete($buf.len())), |$arg| $body }
        );
        ($name:ident, $buf:expr, $len:expr, |$arg:ident| $body:expr) => (
        //#[test]
        pub fn $name() {
            let mut headers = [EMPTY_HEADER; NUM_OF_HEADERS];
            let mut res = Response::new(&mut headers[..]);
            let status = res.parse($buf.as_ref());
            assert_eq!(status, $len);
            closure(res);

            fn closure($arg: Response) {
                $body
            }
        }
        )
    }

    res! {
        test_response_simple,
        b"HTTP/1.1 200 OK\r\n\r\n",
        |res| {
            assert_eq!(res.version.unwrap(), 1);
            assert_eq!(res.code.unwrap(), 200);
            assert_eq!(res.reason.unwrap(), "OK");
        }
    }

    res! {
        test_response_newlines,
        b"HTTP/1.0 403 Forbidden\nServer: foo.bar\n\n",
        |_r| {}
    }

    res! {
        test_response_reason_missing,
        b"HTTP/1.1 200 \r\n\r\n",
        |res| {
            assert_eq!(res.version.unwrap(), 1);
            assert_eq!(res.code.unwrap(), 200);
            assert_eq!(res.reason.unwrap(), "");
        }
    }

    res! {
        test_response_reason_missing_no_space,
        b"HTTP/1.1 200\r\n\r\n",
        |res| {
            assert_eq!(res.version.unwrap(), 1);
            assert_eq!(res.code.unwrap(), 200);
            assert_eq!(res.reason.unwrap(), "");
        }
    }

    res! {
        test_response_reason_missing_no_space_with_headers,
        b"HTTP/1.1 200\r\nFoo: bar\r\n\r\n",
        |res| {
            assert_eq!(res.version.unwrap(), 1);
            assert_eq!(res.code.unwrap(), 200);
            assert_eq!(res.reason.unwrap(), "");
            assert_eq!(res.headers.len(), 1);
            assert_eq!(res.headers[0].name, "Foo");
            assert_eq!(res.headers[0].value, b"bar");
        }
    }

    res! {
        test_response_reason_with_space_and_tab,
        b"HTTP/1.1 101 Switching Protocols\t\r\n\r\n",
        |res| {
            assert_eq!(res.version.unwrap(), 1);
            assert_eq!(res.code.unwrap(), 101);
            assert_eq!(res.reason.unwrap(), "Switching Protocols\t");
        }
    }

    static RESPONSE_REASON_WITH_OBS_TEXT_BYTE: &'static [u8] = b"HTTP/1.1 200 X\xFFZ\r\n\r\n";
    res! {
        test_response_reason_with_obsolete_text_byte,
        RESPONSE_REASON_WITH_OBS_TEXT_BYTE,
        |res| {
            assert_eq!(res.version.unwrap(), 1);
            assert_eq!(res.code.unwrap(), 200);
            // Empty string fallback in case of obs-text
            assert_eq!(res.reason.unwrap(), "");
        }
    }

    res! {
        test_response_reason_with_nul_byte,
        b"HTTP/1.1 200 \x00\r\n\r\n",
        Err(Error::Status),
        |_res| {}
    }

    res! {
        test_response_version_missing_space,
        b"HTTP/1.1",
        Ok(Status::Partial),
        |_res| {}
    }

    res! {
        test_response_code_missing_space,
        b"HTTP/1.1 200",
        Ok(Status::Partial),
        |_res| {}
    }

    res! {
        test_response_empty_lines_prefix_lf_only,
        b"\n\nHTTP/1.1 200 OK\n\n",
        |_res| {}
    }

    res! {
        test_response_no_cr,
        b"HTTP/1.0 200\nContent-type: text/html\n\n",
        |res| {
            assert_eq!(res.version.unwrap(), 0);
            assert_eq!(res.code.unwrap(), 200);
            assert_eq!(res.reason.unwrap(), "");
            assert_eq!(res.headers.len(), 1);
            assert_eq!(res.headers[0].name, "Content-type");
            assert_eq!(res.headers[0].value, b"text/html");
        }
    }

    static RESPONSE_WITH_WHITESPACE_BETWEEN_HEADER_NAME_AND_COLON: &'static [u8] =
        b"HTTP/1.1 200 OK\r\nAccess-Control-Allow-Credentials : true\r\n\r\n";

    //#[test]
    pub fn test_forbid_response_with_whitespace_between_header_name_and_colon() {
        let mut headers = [EMPTY_HEADER; 1];
        let mut response = Response::new(&mut headers[..]);
        let result = response.parse(RESPONSE_WITH_WHITESPACE_BETWEEN_HEADER_NAME_AND_COLON);

        assert_eq!(result, Err(Error::HeaderName));
    }

    //#[test]
    pub fn test_allow_response_with_whitespace_between_header_name_and_colon() {
        let mut headers = [EMPTY_HEADER; 1];
        let mut response = Response::new(&mut headers[..]);
        let result = ParserConfig::default()
            .allow_spaces_after_header_name_in_responses(true)
            .parse_response(&mut response, RESPONSE_WITH_WHITESPACE_BETWEEN_HEADER_NAME_AND_COLON);

        assert_eq!(result, Ok(Status::Complete(60)));
        assert_eq!(response.version.unwrap(), 1);
        assert_eq!(response.code.unwrap(), 200);
        assert_eq!(response.reason.unwrap(), "OK");
        assert_eq!(response.headers.len(), 1);
        assert_eq!(response.headers[0].name, "Access-Control-Allow-Credentials");
        assert_eq!(response.headers[0].value, &b"true"[..]);
    }

    static REQUEST_WITH_WHITESPACE_BETWEEN_HEADER_NAME_AND_COLON: &'static [u8] =
        b"GET / HTTP/1.1\r\nHost : localhost\r\n\r\n";

    //#[test]
    pub fn test_forbid_request_with_whitespace_between_header_name_and_colon() {
        let mut headers = [EMPTY_HEADER; 1];
        let mut request = Request::new(&mut headers[..]);
        let result = request.parse(REQUEST_WITH_WHITESPACE_BETWEEN_HEADER_NAME_AND_COLON);

        assert_eq!(result, Err(Error::HeaderName));
    }

    //#[test]
    pub fn test_chunk_size() {
        assert_eq!(parse_chunk_size(b"0\r\n"), Ok(Status::Complete((3, 0))));
        assert_eq!(parse_chunk_size(b"12\r\nchunk"), Ok(Status::Complete((4, 18))));
        assert_eq!(parse_chunk_size(b"3086d\r\n"), Ok(Status::Complete((7, 198765))));
        assert_eq!(parse_chunk_size(b"3735AB1;foo bar*\r\n"), Ok(Status::Complete((18, 57891505))));
        assert_eq!(parse_chunk_size(b"3735ab1 ; baz \r\n"), Ok(Status::Complete((16, 57891505))));
        assert_eq!(parse_chunk_size(b"77a65\r"), Ok(Status::Partial));
        assert_eq!(parse_chunk_size(b"ab"), Ok(Status::Partial));
        assert_eq!(parse_chunk_size(b"567f8a\rfoo"), Err(InvalidChunkSize));
        assert_eq!(parse_chunk_size(b"567f8a\rfoo"), Err(InvalidChunkSize));
        assert_eq!(parse_chunk_size(b"567xf8a\r\n"), Err(InvalidChunkSize));
        assert_eq!(parse_chunk_size(b"ffffffffffffffff\r\n"), Ok(Status::Complete((18, ::core::u64::MAX))));
        assert_eq!(parse_chunk_size(b"1ffffffffffffffff\r\n"), Err(InvalidChunkSize));
        assert_eq!(parse_chunk_size(b"Affffffffffffffff\r\n"), Err(InvalidChunkSize));
        assert_eq!(parse_chunk_size(b"fffffffffffffffff\r\n"), Err(InvalidChunkSize));
    }

    #[cfg(feature = "std")]
    //#[test]
    pub fn test_std_error() {
        use std::error::Error as StdError;
        let err = Error::HeaderName;
        assert_eq!(err.to_string(), err.description());
    }
}

mod iter {
    //The module is private, so we just copy the entire source code.

    use core::slice;

    pub struct Bytes<'a> {
        slice: &'a [u8],
        pos: usize
    }

    impl<'a> Bytes<'a> {
        #[inline]
        pub fn new(slice: &'a [u8]) -> Bytes<'a> {
            Bytes {
                slice: slice,
                pos: 0
            }
        }

        #[allow(unused)]
        #[inline]
        pub unsafe fn advance(&mut self, n: usize) {
            debug_assert!(self.pos + n <= self.slice.len(), "overflow");
            self.pos += n;
        }

        #[inline]
        pub fn next_8<'b>(&'b mut self) -> Option<Bytes8<'b, 'a>> {
            if self.slice.len() >= self.pos + 8 {
                Some(Bytes8::new(self))
            } else {
                None
            }
        }
    }

    impl<'a> AsRef<[u8]> for Bytes<'a> {
        #[inline]
        fn as_ref(&self) -> &[u8] {
            &self.slice[self.pos..]
        }
    }

    impl<'a> Iterator for Bytes<'a> {
        type Item = u8;

        #[inline]
        fn next(&mut self) -> Option<u8> {
            if self.slice.len() > self.pos {
                let b = unsafe { *self.slice.get_unchecked(self.pos) };
                self.pos += 1;
                Some(b)
            } else {
                None
            }
        }
    }

    pub struct Bytes8<'a, 'b: 'a> {
        bytes: &'a mut Bytes<'b>,
        #[cfg(debug_assertions)]
        pos: usize
    }

    macro_rules! bytes8_methods {
        ($f:ident, $pos:expr) => {
            #[inline]
            pub fn $f(&mut self) -> u8 {
                self.assert_pos($pos);
                let b = unsafe { *self.bytes.slice.get_unchecked(self.bytes.pos) };
                self.bytes.pos += 1;
                b
            }
        };
        () => {
            bytes8_methods!(_0, 0);
            bytes8_methods!(_1, 1);
            bytes8_methods!(_2, 2);
            bytes8_methods!(_3, 3);
            bytes8_methods!(_4, 4);
            bytes8_methods!(_5, 5);
            bytes8_methods!(_6, 6);
            bytes8_methods!(_7, 7);
        }
    }

    impl<'a, 'b: 'a> Bytes8<'a, 'b> {
        bytes8_methods! {}

        #[cfg(not(debug_assertions))]
        #[inline]
        fn new(bytes: &'a mut Bytes<'b>) -> Bytes8<'a, 'b> {
            Bytes8 {
                bytes: bytes,
            }
        }

        #[cfg(debug_assertions)]
        #[inline]
        fn new(bytes: &'a mut Bytes<'b>) -> Bytes8<'a, 'b> {
            Bytes8 {
                bytes: bytes,
                pos: 0,
            }
        }

        #[cfg(not(debug_assertions))]
        #[inline]
        fn assert_pos(&mut self, _pos: usize) {
        }

        #[cfg(debug_assertions)]
        #[inline]
        fn assert_pos(&mut self, pos: usize) {
            assert!(self.pos == pos);
            self.pos += 1;
        }
    }

    //#[cfg(test)]
    pub mod tests {
        use super::Bytes;

        //#[test]
        pub fn test_next_8_too_short() {
            // Start with 10 bytes.
            let slice = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8];
            let mut bytes = Bytes::new(&slice);
            // Skip 3 of them.
            unsafe { bytes.advance(3); }
            // There should be 7 left, not enough to call next_8.
            assert!(bytes.next_8().is_none());
        }

        //#[test]
        pub fn test_next_8_just_right() {
            // Start with 10 bytes.
            let slice = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8];
            let mut bytes = Bytes::new(&slice);
            // Skip 2 of them.
            unsafe { bytes.advance(2); }
            // There should be 8 left, just enough to call next_8.
            let ret = bytes.next_8();
            assert!(ret.is_some());
            let mut ret = ret.unwrap();
            // They should be the bytes starting with 2.
            assert_eq!(ret._0(), 2u8);
            assert_eq!(ret._1(), 3u8);
            assert_eq!(ret._2(), 4u8);
            assert_eq!(ret._3(), 5u8);
            assert_eq!(ret._4(), 6u8);
            assert_eq!(ret._5(), 7u8);
            assert_eq!(ret._6(), 8u8);
            assert_eq!(ret._7(), 9u8);
        }

        //#[test]
        pub fn test_next_8_extra() {
            // Start with 10 bytes.
            let slice = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8];
            let mut bytes = Bytes::new(&slice);
            // Skip 1 of them.
            unsafe { bytes.advance(1); }
            // There should be 9 left, more than enough to call next_8.
            let ret = bytes.next_8();
            assert!(ret.is_some());
            let mut ret = ret.unwrap();
            // They should be the bytes starting with 1.
            assert_eq!(ret._0(), 1u8);
            assert_eq!(ret._1(), 2u8);
            assert_eq!(ret._2(), 3u8);
            assert_eq!(ret._3(), 4u8);
            assert_eq!(ret._4(), 5u8);
            assert_eq!(ret._5(), 6u8);
            assert_eq!(ret._6(), 7u8);
            assert_eq!(ret._7(), 8u8);
        }
    }
}