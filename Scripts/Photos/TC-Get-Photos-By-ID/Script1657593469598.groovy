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
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response3 = WS.sendRequest(findTestObject('Photos/GET - Photos By ID'))

WS.verifyResponseStatusCode(response3, 200, FailureHandling.STOP_ON_FAILURE)

assert response3.getStatusCode() == 200

WS.verifyElementPropertyValue(response3, 'albumId', '1')

WS.verifyElementPropertyValue(response3, 'id', '6')

WS.verifyElementPropertyValue(response3, 'title', 'accusamus ea aliquid et amet sequi nemo')

WS.verifyElementPropertyValue(response3, 'url', 'https://via.placeholder.com/600/56a8c2')

WS.verifyElementPropertyValue(response3, 'thumbnailUrl', 'https://via.placeholder.com/150/56a8c2')
