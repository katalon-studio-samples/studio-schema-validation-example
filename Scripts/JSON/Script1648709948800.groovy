import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable

import org.apache.commons.io.FileUtils
import org.apache.commons.io.FilenameUtils
import org.openqa.selenium.Keys as Keys

res = WS.sendRequest(findTestObject('JSON'))

String jsonSchema = '''
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "array",
  "items": [
    {
      "type": "object",
      "properties": {
        "id": {
          "type": "integer"
        },
        "username": {
          "type": "string"
        },
        "password": {
          "type": "string"
        },
        "gender": {
          "type": "string"
        },
        "age": {
          "type": "integer"
        },
        "avatar": {
          "type": "null"
        }
      },
      "required": [
        "id",
        "username",
        "password",
        "gender",
        "age",
        "avatar"
      ]
    }
  ]
}
'''

WS.validateJsonSchema(res, "example/json/jsonSchema.json");
WS.validateJsonSchema(res, jsonSchema);
WS.validateJsonSchema(res, "example/json/jsonSchemaWrong.json", FailureHandling.CONTINUE_ON_FAILURE);
WS.sendRequestAndVerify(findTestObject('JSON'))