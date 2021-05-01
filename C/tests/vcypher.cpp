#include <gtest/gtest.h>

extern "C" {
#include "../vcypher.h"
}

class VcypherTestSuite : public ::testing::Test {
    void SetUp() override {}

    void TearDown() override {}
};

TEST_F(VcypherTestSuite, testOutput) {
  EXPECT_STREQ(vcypher(L"BFC"), "81156121701");
  EXPECT_STREQ(vcypher(L"*()"), "8123614147318");
}

TEST_F(VcypherTestSuite, testEdgeCasesOutput) {
  EXPECT_STREQ(vcypher(L""), "");
}

TEST_F(VcypherTestSuite, testNonEnglishLettersOutput) {
  EXPECT_STREQ(vcypher(L"ľ"), "623");
  EXPECT_STREQ(vcypher(L"š"), "415");
  EXPECT_STREQ(vcypher(L"č"), "445");
  EXPECT_STREQ(vcypher(L"ť"), "5134");
  EXPECT_STREQ(vcypher(L"ž"), "712");
  EXPECT_STREQ(vcypher(L"ý"), "71");
  EXPECT_STREQ(vcypher(L"á"), "44");
  EXPECT_STREQ(vcypher(L"í"), "612");
  EXPECT_STREQ(vcypher(L"é"), "513");
}