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

res = WS.sendRequest(findTestObject('XML'))

String xml = '''<?xml version="1.0" encoding="utf-8"?>
<List>
    <item>
        <username>James Johnson</username>
        <password>789</password>
        <gender>FEMALE</gender>
        <age>75</age>
        <avatar/>
    </item>
</List>'''

String xmlFile = FileUtils.readFileToString(new File("example/xml/person.xml"));

WS.validateXmlSchema(res, "example/xml/person.xsd");
WS.validateXmlSchema(xml, "example/xml/personWrong.xsd", FailureHandling.CONTINUE_ON_FAILURE);
WS.validateXmlSchema(xmlFile, "example/xml/person.xsd", FailureHandling.STOP_ON_FAILURE);
WS.sendRequestAndVerify(findTestObject('XML'))